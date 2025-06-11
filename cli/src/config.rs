use common::enums::{CropPosition, Game, Operation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct TomlConfig {
  general: GeneralSection,
  game: GameSection,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralSection {
  folder_name: FolderNameSection,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct FolderNameSection {
  background: String,
  center: String,
  cutscene: String,
  foreground0: String,
  foreground1: String,
  foreground2: String,
  foreground3: String,
  foreground4: String,
  foreground5: String,
  full: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct GameSection {
  wuwa: Option<WuWaSection>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct WuWaSection {
  background: LayerConfig,
  center: LayerConfig,
  cutscene: LayerConfig,
  foreground0: LayerConfig,
  foreground1: LayerConfig,
  foreground2: LayerConfig,
  foreground3: LayerConfig,
  foreground4: LayerConfig,
  foreground5: LayerConfig,
  full: LayerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct LayerConfig {
  crop_height: u32,
  crop_position: CropPosition,
  blur: Vec<[u32; 4]>,
}

impl Default for TomlConfig {
  fn default() -> Self {
    TomlConfig { general: GeneralSection::default(), game: GameSection::default() }
  }
}

impl TomlConfig {
  pub fn blur(&self, game: Game, op: Operation) -> Vec<[u32; 4]> {
    match game {
      Game::None => Vec::new(),
      Game::WuWa => {
        let wuwa = self.game.wuwa.as_ref().expect("WuWa config must be present but could not find it.");
        match op {
          Operation::Background => wuwa.background.blur.clone(),
          Operation::Center => wuwa.center.blur.clone(),
          Operation::Cutscene => wuwa.cutscene.blur.clone(),
          Operation::Foreground0 => wuwa.foreground0.blur.clone(),
          Operation::Foreground1 => wuwa.foreground1.blur.clone(),
          Operation::Foreground2 => wuwa.foreground2.blur.clone(),
          Operation::Foreground3 => wuwa.foreground3.blur.clone(),
          Operation::Foreground4 => wuwa.foreground4.blur.clone(),
          Operation::Foreground5 => wuwa.foreground5.blur.clone(),
          Operation::Full => wuwa.full.blur.clone(),
          _ => Vec::new(),
        }
      },
    }
  }

  pub fn crop_height(&self, game: Game, op: Operation) -> u32 {
    match game {
      Game::None => 0,
      Game::WuWa => {
        let wuwa = self.game.wuwa.as_ref().expect("WuWa config must be present but could not find it.");
        match op {
          Operation::Background => wuwa.background.crop_height,
          Operation::Center => wuwa.center.crop_height,
          Operation::Cutscene => wuwa.cutscene.crop_height,
          Operation::Foreground0 => wuwa.foreground0.crop_height,
          Operation::Foreground1 => wuwa.foreground1.crop_height,
          Operation::Foreground2 => wuwa.foreground2.crop_height,
          Operation::Foreground3 => wuwa.foreground3.crop_height,
          Operation::Foreground4 => wuwa.foreground4.crop_height,
          Operation::Foreground5 => wuwa.foreground5.crop_height,
          Operation::Full => wuwa.full.crop_height,
          _ => 0,
        }
      },
    }
  }

  pub fn folder_name(&self, op: Operation) -> Option<String> {
    match op {
      Operation::Background => Some(self.general.folder_name.background.clone()),
      Operation::Center => Some(self.general.folder_name.center.clone()),
      Operation::Cutscene => Some(self.general.folder_name.cutscene.clone()),
      Operation::Foreground0 => Some(self.general.folder_name.foreground0.clone()),
      Operation::Foreground1 => Some(self.general.folder_name.foreground1.clone()),
      Operation::Foreground2 => Some(self.general.folder_name.foreground2.clone()),
      Operation::Foreground3 => Some(self.general.folder_name.foreground3.clone()),
      Operation::Foreground4 => Some(self.general.folder_name.foreground4.clone()),
      Operation::Foreground5 => Some(self.general.folder_name.foreground5.clone()),
      Operation::Full => Some(self.general.folder_name.full.clone()),
      _ => None,
    }
  }
}

impl Default for GeneralSection {
  fn default() -> Self {
    GeneralSection { folder_name: FolderNameSection::default() }
  }
}

impl Default for FolderNameSection {
  fn default() -> Self {
    FolderNameSection {
      background: "CS-Background".into(),
      center: "CS-Center".into(),
      cutscene: "CS-Cutscene".into(),
      foreground0: "CS-Foreground-0".into(),
      foreground1: "CS-Foreground-1".into(),
      foreground2: "CS-Foreground-2".into(),
      foreground3: "CS-Foreground-3".into(),
      foreground4: "CS-Foreground-4".into(),
      foreground5: "CS-Foreground-5".into(),
      full: "CS-Full".into(),
    }
  }
}

impl Default for GameSection {
  fn default() -> Self {
    GameSection { wuwa: Some(WuWaSection::default()) }
  }
}

impl Default for WuWaSection {
  fn default() -> Self {
    WuWaSection {
      background: LayerConfig {
        crop_height: 360,
        crop_position: CropPosition::Bottom,
        blur: vec![[40, 1054, 330, 22], [1733, 1058, 140, 22]],
      },
      center: LayerConfig { crop_height: 200, crop_position: CropPosition::Center, blur: Vec::new() },
      cutscene: LayerConfig { crop_height: 810, crop_position: CropPosition::Center, blur: vec![[1781, 929, 110, 16]] },
      foreground0: LayerConfig {
        crop_height: 310,
        crop_position: CropPosition::Bottom,
        blur: vec![[1733, 1058, 140, 22]],
      },
      foreground1: LayerConfig {
        crop_height: 420,
        crop_position: CropPosition::Bottom,
        blur: vec![[1733, 1058, 140, 22]],
      },
      foreground2: LayerConfig {
        crop_height: 505,
        crop_position: CropPosition::Bottom,
        blur: vec![[1733, 1058, 140, 22]],
      },
      foreground3: LayerConfig {
        crop_height: 580,
        crop_position: CropPosition::Bottom,
        blur: vec![[1733, 1058, 140, 22]],
      },
      foreground4: LayerConfig {
        crop_height: 655,
        crop_position: CropPosition::Bottom,
        blur: vec![[1733, 1058, 140, 22]],
      },
      foreground5: LayerConfig {
        crop_height: 730,
        crop_position: CropPosition::Bottom,
        blur: vec![[1733, 1058, 140, 22]],
      },
      full: LayerConfig {
        crop_height: 0,
        crop_position: CropPosition::Full,
        blur: vec![[40, 1054, 330, 22], [1733, 1058, 140, 22]],
      },
    }
  }
}

impl Default for LayerConfig {
  fn default() -> Self {
    LayerConfig { crop_height: 0, crop_position: CropPosition::Full, blur: Vec::new() }
  }
}
