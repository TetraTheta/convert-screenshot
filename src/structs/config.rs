use crate::structs::enums::{Game, Operation};
use crate::structs::options::parse_tuple;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TomlConfig {
  general: GeneralSection,
  game: Option<GameSection>,
}

#[derive(Serialize, Deserialize)]
pub struct GeneralSection {
  folder_background: String,
  folder_center: String,
  folder_foreground0: String,
  folder_foreground1: String,
  folder_foreground2: String,
  folder_foreground3: String,
  folder_foreground4: String,
  folder_full: String,
}

#[derive(Serialize, Deserialize)]
pub struct GameSection {
  wuwa: Option<WuWaSection>,
}

#[derive(Serialize, Deserialize)]
pub struct WuWaSection {
  crop_background_height: Option<u32>,
  crop_center_height: Option<u32>,
  crop_foreground0_height: Option<u32>,
  crop_foreground1_height: Option<u32>,
  crop_foreground2_height: Option<u32>,
  crop_foreground3_height: Option<u32>,
  crop_foreground4_height: Option<u32>,
  uid_area: Option<String>,
  uid_position: Option<String>,
}

impl TomlConfig {
  pub fn default_for() -> TomlConfig {
    TomlConfig {
      general: GeneralSection {
        folder_background: "CS-Background".into(),
        folder_center: "CS-Center".into(),
        folder_foreground0: "CS-Foreground-0".into(),
        folder_foreground1: "CS-Foreground-1".into(),
        folder_foreground2: "CS-Foreground-2".into(),
        folder_foreground3: "CS-Foreground-3".into(),
        folder_foreground4: "CS-Foreground-4".into(),
        folder_full: "CS-Full".into(),
      },
      game: Some(GameSection {
        wuwa: Some(WuWaSection {
          crop_background_height: Some(360),
          crop_center_height: Some(200),
          crop_foreground0_height: Some(310),
          crop_foreground1_height: Some(420),
          crop_foreground2_height: Some(505),
          crop_foreground3_height: Some(580),
          crop_foreground4_height: Some(655),
          uid_area: Some("144,22".into()),
          uid_position: Some("1744,1059".into()),
        }),
      }),
    }
  }

  pub fn folder_name_for(&self, op: Operation) -> Option<String> {
    match op {
      Operation::Background => Some(self.general.folder_background.clone()),
      Operation::Center => Some(self.general.folder_center.clone()),
      Operation::Foreground0 => Some(self.general.folder_foreground0.clone()),
      Operation::Foreground1 => Some(self.general.folder_foreground1.clone()),
      Operation::Foreground2 => Some(self.general.folder_foreground2.clone()),
      Operation::Foreground3 => Some(self.general.folder_foreground3.clone()),
      Operation::Foreground4 => Some(self.general.folder_foreground4.clone()),
      Operation::Full => Some(self.general.folder_full.clone()),
      _ => None,
    }
  }

  pub fn crop_height_for(&self, game: Game, op: Operation) -> Option<u32> {
    if game != Game::WuWa {
      return None;
    }
    let w_sec = self.game.as_ref()?.wuwa.as_ref()?;
    match op {
      Operation::Background => w_sec.crop_background_height.clone(),
      Operation::Center => w_sec.crop_center_height.clone(),
      Operation::Foreground0 => w_sec.crop_foreground0_height.clone(),
      Operation::Foreground1 => w_sec.crop_foreground1_height.clone(),
      Operation::Foreground2 => w_sec.crop_foreground2_height.clone(),
      Operation::Foreground3 => w_sec.crop_foreground3_height.clone(),
      Operation::Foreground4 => w_sec.crop_foreground4_height.clone(),
      _ => None,
    }
  }

  pub fn uid_area_for(&self, game: Game) -> Option<(u32, u32)> {
    if game != Game::WuWa {
      return None;
    }
    let raw = self.game.as_ref()?.wuwa.as_ref()?.uid_area.as_ref()?;
    parse_tuple(raw.as_str()).ok()
  }

  pub fn uid_pos_for(&self, game: Game) -> Option<(u32, u32)> {
    if game != Game::WuWa {
      return None;
    }
    let raw = self.game.as_ref()?.wuwa.as_ref()?.uid_position.as_ref()?;
    parse_tuple(raw).ok()
  }
}
