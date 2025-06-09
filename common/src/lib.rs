pub mod enums;
pub mod structs;

use std::fs;
use std::path::{Path, PathBuf};

pub fn adjust_extension(file_name: &str) -> String {
  let path = Path::new(file_name);
  if path.extension().is_some() {
    file_name.to_owned()
  } else if cfg!(target_os = "windows") {
    format!("{}.exe", file_name)
  } else {
    file_name.to_owned()
  }
}

pub fn dir_has_image(dir: &Path) -> bool {
  match fs::read_dir(dir) {
    Ok(e) => e.flatten().any(|entry| {
      let path = entry.path();
      entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) && is_image_file(&path)
    }),
    Err(_) => false,
  }
}

/// Returns `true` if the given directory contains any image file (jpg, jpeg, png, webp).
pub fn dir_collect_image(dir: &Path) -> Vec<PathBuf> {
  let mut images = Vec::new();

  if let Ok(entries) = fs::read_dir(dir) {
    for entry in entries.flatten() {
      let path = entry.path();
      if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
        continue;
      }
      if is_image_file(&path) {
        if let Ok(c) = path.canonicalize() {
          images.push(c);
        }
      }
    }
  }
  images
}

/// Returns `Vec<PathBuf>` of absolute paths of image files in the given directory.
/// This does not search subdirectories.
fn is_image_file(p: &Path) -> bool {
  p.extension()
    .and_then(|s| s.to_str())
    .map(|ext| matches!(ext.to_ascii_lowercase().as_str(), "jpg" | "jpeg" | "png" | "webp"))
    .unwrap_or(false)
}
