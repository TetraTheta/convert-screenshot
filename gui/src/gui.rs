use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use eframe::{App, CreationContext, Frame, NativeOptions, run_native};
use egui::text::CCursor;
use egui::text_selection::CCursorRange;
use egui::{
  Align, CentralPanel, CornerRadius, IconData, Label, ProgressBar, ScrollArea, TextEdit, Vec2, ViewportBuilder,
};
use egui_font::set_font;
use native_dialog::{DialogBuilder, MessageLevel};

use crate::MO;

pub enum ImageMsg {
  Done { filename: String },
  Error { text: String },
  Finished,
  Progress { current: usize, total: usize, filename: String },
}

struct CSGuiState {
  abort: bool,
  current: usize,
  current_file: String,
  logs: Vec<String>,
  output: String,
  rx: Receiver<ImageMsg>,
  target: String,
  total: usize,
  autofocused: bool,
}

impl CSGuiState {
  fn new(cc: &CreationContext<'_>, target_list: Vec<PathBuf>, out_dir: PathBuf, rx: Receiver<ImageMsg>) -> Self {
    let total = target_list.len();
    let target = target_list[0].parent().unwrap().to_string_lossy().to_string();
    let output = out_dir.to_string_lossy().to_string();

    set_font(&cc.egui_ctx);

    Self {
      abort: false,
      current: 0,
      current_file: String::new(),
      logs: Vec::new(),
      output,
      rx,
      target,
      total,
      autofocused: false,
    }
  }
}

impl App for CSGuiState {
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
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = Vec2::new(10.0, 10.0);
    ctx.set_style(style);

    CentralPanel::default().show(ctx, |ui| {
      let mut from = remove_unc(self.target.clone());
      let mut to = remove_unc(self.output.clone());
      // TODO: find a way to prevent 'typing' in TextEdit
      ui.horizontal(|ui| {
        ui.label("From: ");
        let mut from_edit = TextEdit::singleline(&mut from).desired_width(f32::INFINITY).auto_scroll(true).show(ui);
        let resp = from_edit.response;
        if !self.autofocused {
          from_edit
            .state
            .cursor
            .set_char_range(Some(CCursorRange::two(CCursor::new(from.len()), CCursor::new(from.len()))));
          // do not set `autofocused` to `true` yet
          from_edit.state.store(ui.ctx(), resp.id);
        }
      });
      ui.horizontal(|ui| {
        ui.label("To:   ");
        let mut to_edit = TextEdit::singleline(&mut to).desired_width(f32::INFINITY).auto_scroll(true).show(ui);
        let resp = to_edit.response;
        if !self.autofocused {
          to_edit.state.cursor.set_char_range(Some(CCursorRange::two(CCursor::new(to.len()), CCursor::new(to.len()))));
          to_edit.state.store(ui.ctx(), resp.id);
          self.autofocused = true;
        }
      });
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

    // update UI as 10FPS
    // without this, the UI update or app close will only happen when the app has 'focus' (mouse hover)
    ctx.request_repaint_after(Duration::from_millis(100));
  }
}

pub fn error_message(s: String) {
  DialogBuilder::message().set_level(MessageLevel::Error).set_title("ERROR").set_text(s).alert().show().unwrap();
}

fn load_icon() -> IconData {
  let icon = include_bytes!("../assets/convert-screenshot-gui.png");
  let img = image::load_from_memory(icon).expect("Failed to load icon").to_rgba8();
  let (w, h) = img.dimensions();
  IconData { rgba: img.into_raw(), width: w, height: h }
}

fn remove_unc(s: String) -> String {
  const PREFIX: &str = r"\\?\";
  if s.starts_with(PREFIX) { (&s[PREFIX.len()..]).parse().unwrap() } else { s }
}

pub fn run_gui(list: Vec<PathBuf>, out_dir: PathBuf, rx: Receiver<ImageMsg>) -> Result<(), eframe::Error> {
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
    centered: true,
    ..Default::default()
  };

  run_native("ConvertScreenshot", opt, Box::new(move |cc| Ok(Box::new(CSGuiState::new(cc, list, out_dir, rx)))))
}
