use std::path::PathBuf;
use std::process::exit;

use fltk::app::{App, Receiver};
use fltk::browser::Browser;
use fltk::enums::{Align, Font};
use fltk::frame::Frame;
use fltk::input::Input;
use fltk::misc::Progress;
use fltk::prelude::*;
use fltk::window::Window;
use fltk::{app, image};
use native_dialog::{DialogBuilder, MessageLevel};

use crate::MO;

pub enum ImageMsg {
  Done { filename: String },
  Error { text: String },
  Finished,
  Progress { current: usize, total: usize, filename: String },
}

pub fn error_message(s: &str) {
  DialogBuilder::message().set_level(MessageLevel::Error).set_title("ERROR").set_text(s).alert().show().unwrap();
}

fn label(x: i32, y: i32, width: i32, height: i32, title: &str) -> Frame {
  Frame::new(x, y, width, height, title).with_align(Align::Left | Align::Inside)
}

pub fn run_gui(from: PathBuf, to: PathBuf, imgs: Vec<PathBuf>, app: App, r: Receiver<ImageMsg>) {
  // FLTK App is already initialized in 'main()' and passed as 'app' due to channel creation
  // FLTK Window
  let mut win = Window::new(100, 100, 484, 461, "ConvertScreenshot");

  // font
  let fonts = app::fonts();
  let font_list = vec![(" Segoe UI Variable Display", 14)];

  if let Some((family, size)) = font_list.iter().find(|(family, _)| fonts.iter().any(|f| f == family)) {
    Font::set_font(Font::Helvetica, &family);
    app::set_font_size(*size);
    // error_message(format!("'{}': {}", family, size));
  } else {
    // error_message(format!("{:?}", fonts));
  }

  // window position
  win.set_pos((app::screen_size().0 / 2.0) as i32, (app::screen_size().1 / 2.0) as i32);

  // window icon
  let icon = image::PngImage::from_data(include_bytes!("../assets/convert-screenshot-gui.png")).unwrap_or_else(|e| {
    error_message(format!("Setting window icon failed\n{}", e).as_str());
    exit(1);
  });
  win.set_icon(Some(icon));

  // display value
  let mo = MO.get().unwrap();
  let game = mo.game;
  let operation = mo.operation;
  let imgs_len = imgs.len();
  let from_string = from.to_string_lossy();
  let from_normalized = from_string.replace("\\", "/");
  let from_str = from_normalized.as_str();
  let to_string = to.to_string_lossy();
  let to_normalized = to_string.replace("\\", "/");
  let to_str = to_normalized.as_str();

  // first row
  let _lbl_from = label(12, 15, 69, 12, "Files From:");
  let mut inp_from = Input::new(87, 12, 385, 21, "");
  inp_from.set_readonly(true);
  inp_from.set_tab_nav(false);
  inp_from.set_value(from_str);
  inp_from.set_position(from_str.len() as i32).unwrap_or_else(|e| {
    error_message(format!("Moving cursor of Input From failed\n{}", e).as_str());
    exit(1);
  });

  // second row
  let _lbl_to = label(12, 42, 69, 12, "Saved To:");
  let mut inp_to = Input::new(87, 39, 385, 21, "");
  inp_to.set_readonly(true);
  inp_to.set_tab_nav(false);
  inp_to.set_value(to_str);
  inp_to.set_position(to_str.len() as i32).unwrap_or_else(|e| {
    error_message(format!("Moving cursor of Input To failed\n{}", e).as_str());
    exit(1);
  });

  // third, fourth row
  let job_info_text = format!("Game: {:?} | Operation: {:?}", game, operation);
  let current_info_text = format!("Total: {} | Current: {}", imgs_len, 0);

  let _lbl_job_info = label(12, 69, 385, 12, job_info_text.as_str()).with_align(Align::Left | Align::Inside);
  let mut lbl_current_info = label(12, 96, 385, 12, current_info_text.as_str());

  // progress bar
  let mut prg_progress = Progress::new(12, 123, 460, 23, "");
  prg_progress.set_minimum(0f64);
  prg_progress.set_maximum(imgs_len as f64);

  // log
  let mut brw_log = Browser::new(12, 157, 460, 292, "");

  win.end();
  win.show();

  // Windows: dark title bar
  #[cfg(target_os = "windows")]
  {
    use windows_sys::Win32::Foundation::HWND;
    use windows_sys::Win32::Graphics::Dwm::{DWMWA_USE_IMMERSIVE_DARK_MODE, DwmSetWindowAttribute};
    use windows_sys::core::BOOL;

    let hwnd = win.raw_handle() as HWND;
    let dark = 1;
    unsafe {
      DwmSetWindowAttribute(
        hwnd,
        DWMWA_USE_IMMERSIVE_DARK_MODE as u32,
        &dark as *const BOOL as *const _,
        size_of_val(&dark) as u32,
      );
    }
  }

  // process message
  while app::wait() {
    if let Some(msg) = r.recv() {
      match msg {
        ImageMsg::Done { filename } => {
          // log
          let size = brw_log.size();
          brw_log.set_text(size, format!("✔ {} DONE", filename).as_str());
          brw_log.select(size);
          brw_log.bottom_line(size);
        },
        ImageMsg::Error { text } => {
          // log
          brw_log.add(format!("✖ {}", text).as_str());
        },
        ImageMsg::Finished => {
          app.quit();
        },
        ImageMsg::Progress { current, total, filename } => {
          // label
          let current_info_text = format!("Total: {} | Current: {}", total, current);
          lbl_current_info.set_label(current_info_text.as_str());
          // progress bar
          let fraction = current as f32 / total as f32;
          let fraction_text = format!("{:.2}%", fraction * 100.0);
          prg_progress.set_value(prg_progress.value() + 1f64);
          prg_progress.set_label(fraction_text.as_str());
          // log
          brw_log.add(format!("→ {}", filename).as_str());
          let size = brw_log.size();
          brw_log.select(size);
          brw_log.bottom_line(size);
        },
      }
    }
  }
}
