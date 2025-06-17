use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

use common::enums::{CropPosition, Game, Operation};
use common::structs::MergedOption;
use image::imageops::{Lanczos3, overlay, resize};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use libblur::FastBlurChannels::Channels4;
use libblur::ThreadingPolicy::Single;
use libblur::{BlurImage, BlurImageMut, BoxBlurParameters, BufferStore, box_blur};
use webp::{Encoder, WebPConfig};

use crate::gui::ImageMsg;

const BLUR_PARAMS: BoxBlurParameters = BoxBlurParameters { x_axis_kernel: 45, y_axis_kernel: 45 };

pub fn process_image(images: Vec<PathBuf>, mo: &MergedOption, out_dir: PathBuf, tx: Sender<ImageMsg>) {
  let total = images.len();

  // only used for blur at the outside of loop
  let mut tmp_src = Vec::new();
  let mut tmp_dst = Vec::new();

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

      // blur
      if mo.should_blur(w) {
        for area in &mo.blur {
          let (x, y, bw, bh) = (area[0], area[1], area[2], area[3]);
          if x + bw <= w && y + bh <= h {
            tmp_src.clear();
            tmp_dst.clear();
            tmp_src.extend_from_slice(&img.crop_imm(x, y, bw, bh).to_rgba8().into_raw());
            let src = BlurImage::borrow(&tmp_src, bw, bh, Channels4);
            tmp_dst.resize((bw * bh * 4) as usize, 0);
            let mut dst = BlurImageMut {
              data: BufferStore::from(BufferStore::Owned(tmp_dst.clone())),
              width: bw,
              height: bh,
              stride: bw * 4,
              channels: Channels4,
            };
            box_blur(&src, &mut dst, BLUR_PARAMS, Single).expect("Failed to blur image");
            let buf = dst.data.borrow();
            let layer: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(bw, bh, buf.to_vec()).unwrap();
            overlay(&mut img, &DynamicImage::ImageRgba8(layer), x.into(), y.into());
          }
        }
      }

      // crop
      img = match mo.crop_pos {
        CropPosition::Bottom => img.crop_imm(0, h - mo.crop_height, w, mo.crop_height),
        CropPosition::Center => {
          let top = (h.saturating_sub(mo.crop_height)) / 2;
          img.crop_imm(0, top, w, mo.crop_height)
        },
        CropPosition::Full => img,
      };

      // resize
      if mo.should_resize(w) {
        let ratio = img.height() as f32 / img.width() as f32; // using (maybe) cropped img value!
        let new_h = (mo.width_to as f32 * ratio) as u32;
        img = DynamicImage::ImageRgba8(resize(&img, mo.width_to, new_h, Lanczos3))
      }
    };

    // manually create WebPConfig with the value of PICTURE preset
    let mut config = WebPConfig::new().unwrap();
    config.quality = 85.0;
    config.sns_strength = 80; // PICTURE
    config.filter_sharpness = 4; // PICTURE
    config.filter_strength = 35;
    config.preprocessing = 2;
    config.method = 6;
    config.thread_level = 1;
    config.pass = 4;

    // encode to webp with config
    let webp = Encoder::from_image(&img).unwrap().encode_advanced(&config).unwrap();

    // save
    let dst = out_dir.join(f.file_stem().unwrap()).with_extension("webp");
    if let Err(e) = fs::write(&dst, &*webp) {
      tx.send(ImageMsg::Error { text: format!("Failed to write '{}': {}", dst.display(), e) })
        .expect("Failed to send error message");
      // silently skip to next image
      continue;
    }
    tx.send(ImageMsg::Done { filename }).ok();
  }

  tx.send(ImageMsg::Finished).ok();
}
