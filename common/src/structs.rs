use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::enums::{CropPosition, Game, Operation};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MergedOption {
  pub crop_height: u32,
  pub crop_pos: CropPosition,
  pub game: Game,
  pub operation: Operation,
  pub target: PathBuf,
  pub uid_area: (u32, u32),
  pub uid_pos: (u32, u32),
  pub width_from: u32,
  pub width_to: u32,
}

impl MergedOption {
  pub fn should_blur(&self, img_width: u32) -> bool {
    if self.game == Game::None {
      return false;
    }
    img_width == self.width_from
  }

  pub fn should_resize(&self, img_width: u32) -> bool {
    if self.game == Game::None {
      return false;
    }
    match self.operation {
      Operation::Full => img_width > self.width_to,
      _ => img_width == self.width_from,
    }
  }
}
