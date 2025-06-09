mod config;
mod options;

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio, exit};
use std::{env, fs};

use clap::Parser;
use common::enums::{Game, Operation};
use common::structs::MergedOption;
use common::{adjust_extension, dir_has_image};
use config::TomlConfig;

use crate::options::{Options, merge_options};

fn main() {
  // prepare TOML config
  let bin_path = env::current_exe().expect("Could not get current executable path");
  let toml_path = bin_path.with_extension("toml");
  let config: TomlConfig = if toml_path.exists() {
    let mut buf: String = String::new();
    File::open(&toml_path).and_then(|mut f| f.read_to_string(&mut buf)).unwrap_or_else(|e| {
      eprintln!("Failed to read TOML file '{}': {}", toml_path.display(), e);
      exit(1)
    });
    toml::from_str(&buf).unwrap_or_else(|e| {
      eprintln!("Failed to parse TOML file '{}': {}", toml_path.display(), e);
      exit(1);
    })
  } else {
    // create default and write it out
    let default_config = TomlConfig::default_for();
    let toml_str = toml::to_string(&default_config).unwrap();
    File::create(&toml_path).and_then(|mut f| f.write_all(toml_str.as_bytes())).unwrap_or_else(|e| {
      eprintln!("Failed to write default TOML file '{}': {}", toml_path.display(), e);
      exit(1);
    });
    default_config
  };

  // write TOML config for empty / missing key/value
  let toml_str = toml::to_string_pretty(&config).unwrap(); // very unlikely to error
  if let Err(e) = fs::write(&toml_path, toml_str) {
    eprintln!("Failed to write TOML file '{}': {}", toml_path.display(), e);
  }

  // parse CLI
  let cli = Options::parse();

  let target_dir = cli.target.clone();
  if !target_dir.exists() {
    eprintln!("Target directory '{}' does not exist", target_dir.display());
    exit(1);
  }
  if !target_dir.is_dir() {
    eprintln!("Target directory '{}' is not a directory", target_dir.display());
    exit(1);
  }

  // validate Options
  if cli.operation != Operation::Full && cli.operation != Operation::CreateDirectory {
    if cli.game == Game::None {
      eprintln!(
        "When Operation {:?} is specified, you must also set Game to something other than 'None'.",
        cli.operation
      );
      exit(1);
    }
  }

  // handle Operation
  match cli.operation {
    // CreateDirectory
    Operation::CreateDirectory => {
      for &op in &[
        Operation::Background,
        Operation::Center,
        Operation::Foreground0,
        Operation::Foreground1,
        Operation::Foreground2,
        Operation::Foreground3,
        Operation::Foreground4,
        Operation::Foreground5,
        Operation::Full,
      ] {
        if let Some(folder_name) = config.folder_name_for(op) {
          let dir_path = target_dir.join(folder_name);
          if let Err(e) = fs::create_dir_all(&dir_path) {
            eprintln!("Failed to create directory '{}': {}", dir_path.display(), e);
          }
        }
      }
      println!("Created directory under '{}'.", target_dir.display());
      return;
    },
    // All
    Operation::All => {
      let mut sub_options = Vec::new();
      for &op in &[
        Operation::Background,
        Operation::Center,
        Operation::Foreground0,
        Operation::Foreground1,
        Operation::Foreground2,
        Operation::Foreground3,
        Operation::Foreground4,
        Operation::Foreground5,
        Operation::Full,
      ] {
        if let Some(folder_name) = config.folder_name_for(op) {
          let sub_target = target_dir.join(folder_name);
          if !sub_target.exists() {
            continue;
          }
          let has_image = dir_has_image(&sub_target);
          if !has_image {
            let _ = fs::remove_dir_all(&sub_target);
            continue;
          }

          let eff = merge_options(&cli, &config, &sub_target, cli.game, op);
          sub_options.push(eff);
        }
      }
      for mo in sub_options {
        run_gui(&mo);
      }
      return;
    },
    // others
    op @ (Operation::Background
    | Operation::Center
    | Operation::Foreground0
    | Operation::Foreground1
    | Operation::Foreground2
    | Operation::Foreground3
    | Operation::Foreground4
    | Operation::Foreground5
    | Operation::Full) => {
      let mut new_dir = None;
      if let Some(folder_name) = config.folder_name_for(op) {
        let dir1 = target_dir.join(folder_name);
        if dir1.exists() && dir_has_image(&dir1) {
          new_dir = Some(dir1);
        }
      }
      if new_dir.is_none() {
        if dir_has_image(&target_dir) {
          new_dir = Some(target_dir.clone());
        }
      }
      let final_target = if let Some(d) = new_dir {
        d
      } else {
        eprintln!(
          "No image files were found in '{}' or its '{}' directory.",
          target_dir.display(),
          config.folder_name_for(op).unwrap_or("<unknown>".to_string())
        );
        exit(1);
      };

      let mo = merge_options(&cli, &config, &final_target, cli.game, op);
      run_gui(&mo);
      return;
    },
  }
}

fn run_gui(jd: &MergedOption) {
  let bin_self = env::current_exe().expect("Could not get current exe path");
  let dir_parent = bin_self.parent().unwrap_or_else(|| Path::new("."));
  let bin_gui = dir_parent.join(adjust_extension("cs-gui")); // hard-coded GUI program name for speed

  if !bin_gui.exists() || !bin_gui.is_file() {
    eprintln!("GUI program, '{}' does not exist, or is not a file.", bin_gui.display());
    exit(1);
  }

  let mut child = Command::new(&bin_gui)
    .stdin(Stdio::piped())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .spawn()
    .unwrap_or_else(|e| {
      eprintln!("Failed to execute process '{}': {}", bin_gui.display(), e);
      exit(1);
    });

  if let Some(mut stdin) = child.stdin.take() {
    serde_json::to_writer(&mut stdin, &jd).unwrap_or_else(|e| {
      eprintln!("Failed to write JSON to {} stdin: {}", bin_gui.display(), e);
    });
  }
}
