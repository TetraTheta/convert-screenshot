use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

use common::enums::{CropPosition, Game, Operation};
use common::structs::MergedOption;
use image::imageops::{Lanczos3, blur};
use image::{DynamicImage, GenericImageView};
use webp::Encoder;

use crate::gui::ImageMsg;

pub fn process_image(images: Vec<PathBuf>, mo: &MergedOption, tx: Sender<ImageMsg>) {
  let total = images.len();

  for (i, f) in images.iter().enumerate() {
    let filename = f.file_name().unwrap().to_string_lossy().to_string();
    tx.send(ImageMsg::Progress { current: i + 1, total, filename: filename.clone() }).ok();

    // load image
    let mut img = match image::open(f) {
      Ok(i) => i,
      Err(e) => {
        tx.send(ImageMsg::Error { text: format!("Failed to open '{}': {}", f.display(), e) })
          .expect("Failed to send error message");
        // silently skip to next image
        continue;
      },
    };

    if mo.game != Game::None {
      let (w, h) = img.dimensions();

      if mo.operation != Operation::Full && w != mo.width_from {
        tx.send(ImageMsg::Error { text: format!("Expected width is {} but got {}: {}", mo.width_from, w, filename) })
          .expect("Failed to send error message");
        // silently skip to next image
        continue;
      }

      // blur UID
      if mo.should_blur(w) && mo.uid_area != (0, 0) && mo.uid_pos != (0, 0) {
        let sub = img.crop_imm(mo.uid_pos.0, mo.uid_pos.1, mo.uid_area.0, mo.uid_area.1);
        let blurred = blur(&sub, 6.7); // most similar value for 'boxblur=3:15'
        image::imageops::overlay(&mut img, &blurred, mo.uid_pos.0 as i64, mo.uid_pos.1 as i64);
      }

      // crop
      img = match mo.crop_pos {
        CropPosition::Bottom => img.crop_imm(0, h - mo.crop_height, w, mo.crop_height),
        CropPosition::Center => {
          let y = (h.saturating_sub(mo.crop_height)) / 2;
          img.crop_imm(0, y, w, mo.crop_height)
        },
        CropPosition::Full => img,
      };

      // resize
      if mo.should_resize(w) {
        let ratio = img.height() as f32 / img.width() as f32; // using (maybe) cropped img value!
        let new_h = (mo.width_to as f32 * ratio) as u32;
        img = DynamicImage::ImageRgba8(image::imageops::resize(&img, mo.width_to, new_h, Lanczos3))
      }
    };

    // encode to webp
    let webp = Encoder::from_image(&img).unwrap().encode(85.0);

    // save
    // I don't think using 'unwrap()' will cause problem in here
    let dir_conv = f.parent().unwrap().join("converted");
    fs::create_dir_all(&dir_conv).ok();
    let path_result = dir_conv.join(f.with_extension("webp").file_name().unwrap());
    match fs::write(&path_result, &*webp) {
      Ok(_) => {
        tx.send(ImageMsg::Done { filename }).ok();
      },
      Err(e) => {
        tx.send(ImageMsg::Error { text: format!("Failed to write '{}': {}", path_result.display(), e) })
          .expect("Failed to send error message");
        // silently skip to next image
        continue;
      },
    }
  }

  tx.send(ImageMsg::Finished).ok();
}
