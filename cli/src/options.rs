use std::env;
use std::path::{Path, PathBuf};

use clap::Parser;
use common::enums::{CropPosition, Game, Operation};
use common::structs::MergedOption;

use crate::config::TomlConfig;

#[derive(Parser)]
#[command(version, about)]
pub struct Options {
  /// Operation to take on to the screenshots.
  /// If you specify anything other than 'Full' or 'CreateDirectory', you must also set '-g|--game' to other than
  /// 'None'.
  #[command(subcommand)]
  pub operation: Operation,

  /// Target directory (default: current working directory)
  #[arg(global = true, value_name = "TARGET", default_value = get_cwd().into_os_string(), index = 1)]
  pub target: PathBuf,

  /// Manual override: Area for blur, as 'x,y,width,height'
  #[arg(long, global = true, value_parser = parse_tuple)]
  pub blur: Option<Vec<[u32; 4]>>,

  /// Manual override: crop height in pixel
  #[arg(long, global = true)]
  pub crop_height: Option<u32>,

  /// Manual override: crop position
  #[arg(long, global = true, value_enum)]
  pub crop_pos: Option<CropPosition>,

  /// Game that the screenshots are taken from
  #[arg(short = 'g', long, global = true, value_enum, default_value_t = Game::None)]
  pub game: Game,

  /// Manual override: Width of original image
  #[arg(long, global = true)]
  pub width_from: Option<u32>,

  /// Manual override: Width of converted image
  #[arg(long, global = true)]
  pub width_to: Option<u32>,
}

fn get_cwd() -> PathBuf {
  env::current_dir().unwrap()
}

pub fn merge_options(
  opt: &Options,
  config: &TomlConfig,
  target: &Path,
  game: Game,
  op: Operation,
  save_at_parent: bool,
) -> MergedOption {
  // blur
  let blur = opt.blur.clone().unwrap_or_else(|| config.blur(game, op));

  // crop_height
  let crop_height = match op {
    Operation::All | Operation::CreateDirectory | Operation::Full => 0,
    _ => opt
      .crop_height
      .or_else(|| Some(config.crop_height(game, op)))
      .expect("'crop_height' must come from either CLI or TOML."),
  };

  // crop_pos
  let crop_pos = opt.crop_pos.unwrap_or_else(|| config.crop_position(game, op));

  // width_from, width_to
  let (default_width_from, default_width_to) = if game != Game::None { (1920, 1280) } else { (0, 0) };
  let width_from = opt.width_from.unwrap_or(default_width_from);
  let width_to = opt.width_to.unwrap_or(default_width_to);

  MergedOption {
    blur,
    crop_height,
    crop_pos,
    game,
    save_at_parent,
    operation: op,
    target: target.to_path_buf(),
    width_from,
    width_to,
  }
}

pub fn parse_tuple(s: &str) -> Result<[u32; 4], String> {
  let parts: Vec<&str> = s.split(',').collect();
  if parts.len() != 4 {
    return Err("Must be four unsigned integers separated by commas, e.g. \"10,20,31,42\"".into());
  }
  let mut nums = [0u32; 4];
  for (i, part) in parts.iter().enumerate() {
    nums[i] =
      part.trim().parse::<u32>().map_err(|_| format!("Failed to parse integer at position {}: '{}'", i + 1, part))?;
  }
  Ok(nums)
}
