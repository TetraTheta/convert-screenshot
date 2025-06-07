use crate::structs::config::TomlConfig;
use crate::structs::enums::{Game, Operation};
use crate::structs::options::{merge_options, JobData, Options};
use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{exit, Command, Stdio};
use std::{env, fs};

pub mod structs;

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
        "When Operation {} is specified, you must also set Game to something other than 'None'.",
        format!("{:?}", cli.operation)
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
        let jd = JobData {
          crop_height: mo.crop_height,
          crop_pos: mo.crop_pos,
          operation: mo.operation,
          target: mo.target.clone(),
          uid_area: mo.uid_area,
          uid_pos: mo.uid_pos,
          width_from: mo.width_from,
          width_to: mo.width_to,
        };
        run_gui(&jd);
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
      let jd = JobData {
        crop_height: mo.crop_height,
        crop_pos: mo.crop_pos,
        operation: mo.operation,
        target: mo.target.clone(),
        uid_area: mo.uid_area,
        uid_pos: mo.uid_pos,
        width_from: mo.width_from,
        width_to: mo.width_to,
      };
      run_gui(&jd);
      return;
    },
  }
}

fn dir_has_image(dir: &Path) -> bool {
  if !dir.is_dir() {
    return false;
  }
  match fs::read_dir(dir) {
    Ok(mut entries) => entries.any(|res| {
      if let Ok(entry) = res {
        if let Some(ext) = entry.path().extension() {
          let el = ext.to_string_lossy().to_lowercase();
          return el == "jpg" || el == "jpeg" || el == "png" || el == "webp";
        }
      }
      false
    }),
    Err(_) => false,
  }
}

fn run_gui(jd: &JobData) {
  let bin_self = env::current_exe().expect("Could not get current exe path");
  let dir_parent = bin_self.parent().unwrap_or_else(|| Path::new("."));
  let stem = bin_self.file_stem().and_then(|s| s.to_str()).expect("Executable file name was not valid UTF-8");
  let expected = format!("{}-gui.exe", stem);

  let bin_gui = fs::read_dir(dir_parent)
    .unwrap_or_else(|e| {
      eprintln!("Failed to read '{}': {}", dir_parent.display(), e);
      exit(1);
    })
    .filter_map(Result::ok)
    .find(|e| e.file_name().to_string_lossy().eq_ignore_ascii_case(&expected))
    .map(|e| e.path())
    .unwrap_or_else(|| {
      eprintln!("Could not find any file named '{}' (case-insensitive) in '{}'.", expected, dir_parent.display());
      exit(1);
    });

  let json_payload = serde_json::to_string(jd).unwrap();
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
    stdin.write_all(json_payload.as_bytes()).unwrap_or_else(|e| {
      eprintln!("Failed to write JSON to {} stdin: {}", bin_gui.display(), e);
    });
  }
}
