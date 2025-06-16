#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

mod gui;
mod image;

use std::fs::create_dir_all;
use std::io::{Read, stdin};
use std::path::PathBuf;
use std::process::exit;
use std::sync::{OnceLock, mpsc};
use std::thread;

use common::dir_collect_image;
use common::structs::MergedOption;

use crate::gui::{error_message, run_gui};
use crate::image::process_image;

static MO: OnceLock<MergedOption> = OnceLock::new();

fn output_dir(mo: &MergedOption) -> PathBuf {
  let target = &mo.target;
  let candidate = if mo.save_at_parent {
    target.parent().and_then(|p| if p.parent().is_some() { Some(p.to_path_buf()) } else { None }).unwrap_or_else(|| {
      let mut s = target.clone();
      s.set_file_name(format!("{}-converted", target.file_name().unwrap().to_string_lossy()));
      s
    })
  } else {
    target.join("converted")
  };

  create_dir_all(&candidate).ok();
  candidate
}

fn main() {
  // parse JSON data
  let mut input = String::new();
  if stdin().read_to_string(&mut input).unwrap_or(0) == 0 {
    // no input
    error_message("No JSON input were found.".to_string());
    exit(1);
  }

  // parse to MergedOption
  let mo: MergedOption = match serde_json::from_str(&input) {
    Ok(v) => v,
    Err(e) => {
      error_message(format!("Failed to parse JSON data:\n{}", e));
      exit(1);
    },
  };

  MO.set(mo.clone()).expect("MO already initialized");

  // get image file list
  let imgs = dir_collect_image(&mo.target.to_path_buf());
  if imgs.is_empty() {
    error_message(format!("There is no image file in '{:?}'", &mo.target));
    exit(1);
  }

  let out_dir = output_dir(&mo);
  let out_dir_c = out_dir.clone();

  // spawn image processing thread
  let imgs_2 = imgs.clone();
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    process_image(imgs_2, &mo, out_dir, tx);
  });

  // spawn GUI and pass receiver
  if let Err(e) = run_gui(imgs, out_dir_c, rx) {
    error_message(format!("GUI error: {}", e));
    exit(1);
  }
}
