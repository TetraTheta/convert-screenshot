use crate::structs::config::TomlConfig;
use crate::structs::enums::{CropPosition, Game, Operation};
use clap::Parser;
use serde::Serialize;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(version, about)]
pub struct Options {
  /// Manual override: crop height in pixel  
  #[arg(long)]
  pub crop_height: Option<u32>,

  /// Manual override: crop position
  #[arg(long, value_enum)]
  pub crop_pos: Option<CropPosition>,

  /// Game that the screenshots are taken from
  #[arg(short = 'g', long, value_enum, default_value_t = Game::None)]
  pub game: Game,

  /// Operation to take on to the screenshots.
  /// If you specify anything other than 'Full' or 'CreateDirectory', you must also set '-g|--game' to other than 'None'.
  #[arg(short = 'o', long, value_enum, default_value_t = Operation::Full)]
  pub operation: Operation,

  /// Target directory (default: current working directory)
  #[arg(default_value = get_cwd().into_os_string())]
  pub target: PathBuf,

  /// Manual override: UID area as 'x,y'
  #[arg(long, value_parser = parse_tuple)]
  pub uid_area: Option<(u32, u32)>,

  /// Manual override: UID position as 'x,y'
  #[arg(long, value_parser = parse_tuple)]
  pub uid_pos: Option<(u32, u32)>,

  /// Manual override: Width of original image
  #[arg(long)]
  pub width_from: Option<u32>,

  /// Manual override: Width of converted image
  #[arg(long)]
  pub width_to: Option<u32>,
}

fn get_cwd() -> PathBuf {
  env::current_dir().unwrap()
}

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

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobData {
  pub crop_height: u32,
  pub crop_pos: CropPosition,
  pub operation: Operation,
  pub target: PathBuf,
  pub uid_area: (u32, u32),
  pub uid_pos: (u32, u32),
  pub width_from: u32,
  pub width_to: u32,
}

pub fn merge_options(opt: &Options, config: &TomlConfig, target: &Path, game: Game, op: Operation) -> MergedOption {
  // crop_height
  let crop_height = match op {
    Operation::All | Operation::CreateDirectory | Operation::Full => 0,
    _ => opt
      .crop_height
      .or_else(|| config.crop_height_for(game, op))
      .expect("'Crop Height' must be provided either on the CLI or in the TOML config."),
  };

  // crop_pos
  let crop_pos = opt.crop_pos.unwrap_or_else(|| match op {
    Operation::Full => CropPosition::Full,
    Operation::Center => CropPosition::Center,
    _ => CropPosition::Bottom,
  });

  // uid_area
  let uid_area = opt.uid_area.or_else(|| config.uid_area_for(game)).unwrap_or((0, 0));

  // uid_pos
  let uid_pos = opt.uid_pos.or_else(|| config.uid_pos_for(game)).unwrap_or((0, 0));

  // width_from, width_to
  let (width_from, width_to) =
    opt.width_from.zip(opt.width_to).unwrap_or_else(|| if op != Operation::Full { (1920, 1280) } else { (0, 0) });

  MergedOption {
    crop_height,
    crop_pos,
    game,
    operation: op,
    target: target.to_path_buf(),
    uid_area,
    uid_pos,
    width_from,
    width_to,
  }
}

pub fn parse_tuple(s: &str) -> Result<(u32, u32), String> {
  let parts: Vec<_> = s.split(",").collect();
  if parts.len() != 2 {
    return Err("Must be two unsigned integers separated by comma, e.g. \"144,22\"".into());
  }
  let a = parts[0].parse::<u32>().map_err(|_| "Failed to parse first integer".to_string())?;
  let b = parts[1].parse::<u32>().map_err(|_| "Failed to parse second integer".to_string())?;
  Ok((a, b))
}
