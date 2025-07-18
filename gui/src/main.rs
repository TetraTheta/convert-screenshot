#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

mod gui;
mod image;

use std::fs::create_dir_all;
use std::io::{Read, stdin};
use std::path::PathBuf;
use std::process::exit;
use std::sync::OnceLock;
use std::thread;

use common::dir_collect_image;
use common::structs::MergedOption;
use fltk::app;
use fltk::app::{Receiver, Sender};
use fltk_theme::{ColorTheme, color_themes};

use crate::gui::{ImageMsg, error_message, run_gui};
use crate::image::process_image;

static MO: OnceLock<MergedOption> = OnceLock::new();

fn main() {
  // parse JSON data
  let mut input = String::new();
  if stdin().read_to_string(&mut input).unwrap_or(0) == 0 {
    // no input
    error_message("No JSON input were found.");
    exit(1);
  }

  // parse to MergedOption
  let mo: MergedOption = match serde_json::from_str(&input) {
    Ok(v) => v,
    Err(e) => {
      error_message(format!("Failed to parse JSON data:\n{}", e).as_str());
      exit(1);
    },
  };
  MO.set(mo.clone()).expect("MO already initialized");

  // get image file list
  let imgs = dir_collect_image(&mo.target.to_path_buf());
  if imgs.is_empty() {
    error_message(format!("There is no image file in '{:?}'", &mo.target).as_str());
    exit(1);
  }

  // FLTK App must be pre-initialized here, which I personally don't like :(
  let app = app::App::default().with_scheme(app::Scheme::Gtk).load_system_fonts();
  // don't use widget theme because they don't implement 'OS_BUTTON_DOWN_BOX' and 'OS_BUTTON_DOWN_FRAME'
  let color_theme = ColorTheme::new(&color_themes::fleet::DRACULA);
  color_theme.apply();

  // FLTK Channel
  let (s, r): (Sender<ImageMsg>, Receiver<ImageMsg>) = app::channel();

  let from = mo.target.clone();
  let to = output_dir(&mo);

  // spawn image processing thread
  let imgs_t = imgs.clone();
  let to_t = to.clone();
  thread::spawn(move || {
    process_image(imgs_t, &mo, to_t, s);
  });

  // spawn GUI and pass receiver
  // This will handle Window, not App
  run_gui(from, to, imgs, app, r);
}

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
