#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

mod gui;
mod image;

use std::io::{Read, stdin};
use std::process::exit;
use std::sync::{OnceLock, mpsc};
use std::thread;

use common::dir_collect_image;
use common::structs::MergedOption;

use crate::gui::{error_message, run_gui};
use crate::image::process_image;

static MO: OnceLock<MergedOption> = OnceLock::new();

fn main() {
  // parse JSON data
  let mut input = String::new();
  if stdin().read_to_string(&mut input).unwrap_or(0) == 0 {
    // no input
    let _ = error_message("No JSON input were found.".to_string());
    exit(1);
  }

  // parse to MergedOption
  let mo: MergedOption = match serde_json::from_str(&input) {
    Ok(v) => v,
    Err(e) => {
      let _ = error_message(format!("Failed to parse JSON data:\n{}", e));
      exit(1);
    },
  };

  MO.set(mo.clone()).expect("MO already initialized");

  // get image file list
  let imgs = dir_collect_image(&mo.target.to_path_buf());
  if imgs.is_empty() {
    let _ = error_message(format!("There is no image file in '{:?}'", &mo.target));
    exit(1);
  }

  // spawn image processing thread
  let imgs_2 = imgs.clone();
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    process_image(imgs_2, &mo, tx);
  });

  // spawn GUI and pass receiver
  run_gui(imgs, rx);
}
