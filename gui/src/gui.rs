use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use eframe::{App, Frame, NativeOptions, run_native};
use egui::{
  Align, CentralPanel, CornerRadius, IconData, Label, ProgressBar, ScrollArea, TextEdit, Vec2, ViewportBuilder,
};
use native_dialog::{DialogBuilder, MessageLevel};

use crate::MO;

pub enum ImageMsg {
  Done { filename: String },
  Error { text: String },
  Finished,
  Progress { current: usize, total: usize, filename: String },
}

struct AppState {
  abort: bool,
  current: usize,
  current_file: String,
  logs: Vec<String>,
  rx: Receiver<ImageMsg>,
  target: String,
  total: usize,
}

impl AppState {
  fn new(target_list: Vec<PathBuf>, rx: Receiver<ImageMsg>) -> Self {
    let total = target_list.len();
    let target = target_list[0].parent().unwrap().to_string_lossy().to_string();

    Self { abort: false, current: 0, current_file: String::new(), logs: Vec::new(), rx, target, total }
  }
}

impl App for AppState {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
    while let Ok(msg) = self.rx.try_recv() {
      match msg {
        ImageMsg::Done { filename } => {
          if let Some(last) = self.logs.last_mut() {
            *last = format!("Processing {} DONE", filename)
          }
        },
        ImageMsg::Error { text } => {
          error_message(text);
        },
        ImageMsg::Finished => exit(1),
        ImageMsg::Progress { current, total: _total, filename } => {
          self.current = current;
          self.current_file = filename.clone();
          self.logs.push(format!("Processing {}", filename));
        },
      }
    }

    // set margin
    // TODO: find a way to use font with CJK character, without embedding it
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = Vec2::new(10.0, 10.0);
    ctx.set_style(style);

    CentralPanel::default().show(ctx, |ui| {
      // remove '\\?\' from path
      let mut path = self.target.clone();
      if path.starts_with(r"\\?\") {
        path = path[r"\\?\".len()..].to_string();
      }
      // TODO: find a way to prevent 'typing' in TextEdit
      ui.add(TextEdit::singleline(&mut path).desired_width(f32::INFINITY));
      ui.add(
        Label::new(format!("Game: {:?}, Operation: {:?}", MO.get().unwrap().game, MO.get().unwrap().operation))
          .selectable(false),
      );
      ui.add(Label::new(format!("Total: {} | Current: {}", self.total, self.current)).selectable(false));
      let fraction = self.current as f32 / self.total as f32;
      ui.add(ProgressBar::new(fraction).corner_radius(CornerRadius::same(4)).text(format!("{:.2}%", fraction * 100.0)));
      ScrollArea::vertical().max_width(f32::INFINITY).auto_shrink(false).show(ui, |ui| {
        for log in &self.logs {
          ui.add(Label::new(log).selectable(true));
        }
        ui.scroll_to_cursor(Some(Align::BOTTOM));
      })
    });

    if self.abort {
      exit(1)
    }

    // update UI in every 100ms
    // this is mandatory to prevent ui update or app close only happen when the app is focused
    ctx.request_repaint_after(Duration::from_millis(100));
  }
}

pub fn error_message(s: String) {
  DialogBuilder::message().set_level(MessageLevel::Error).set_title("ERROR").set_text(s).alert().show().unwrap();
}

fn load_icon() -> IconData {
  let icon = include_bytes!("../assets/icon.png");
  let img = image::load_from_memory(icon).expect("Failed to load icon").to_rgba8();
  let (w, h) = img.dimensions();
  IconData { rgba: img.into_raw(), width: w, height: h }
}

pub fn run_gui(list: Vec<PathBuf>, rx: Receiver<ImageMsg>) {
  let icon = Arc::new(load_icon());
  let opt = NativeOptions {
    viewport: ViewportBuilder {
      active: Some(true),
      maximize_button: Some(false),
      drag_and_drop: Some(false),
      max_inner_size: Some(Vec2::new(500.0, 500.0)),
      min_inner_size: Some(Vec2::new(500.0, 500.0)),
      close_button: Some(true),
      minimize_button: Some(true),
      icon: Some(icon),
      resizable: Some(false),
      title: Some("ConvertScreenshot".to_string()),
      ..Default::default()
    },
    ..Default::default()
  };

  run_native("ConvertScreenshot", opt, Box::new(move |_cc| Ok(Box::new(AppState::new(list, rx)))))
    .expect("Failed to open GUI");
}
