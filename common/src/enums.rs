#[cfg(feature = "cli")]
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(ValueEnum))]
pub enum CropPosition {
  Bottom,
  Center,
  Full,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(ValueEnum))]
pub enum Operation {
  #[cfg_attr(feature = "cli", clap(aliases = ["a"]))]
  All,
  #[cfg_attr(feature = "cli", clap(aliases = ["b", "bg"]))]
  Background,
  #[cfg_attr(feature = "cli", clap(aliases = ["c"]))]
  Center,
  #[cfg_attr(feature = "cli", clap(aliases = ["cd"]))]
  CreateDirectory,
  #[cfg_attr(feature = "cli", clap(aliases = ["f0", "fg0"]))]
  Foreground0,
  #[cfg_attr(feature = "cli", clap(aliases = ["f1", "fg1"]))]
  Foreground1,
  #[cfg_attr(feature = "cli", clap(aliases = ["f2", "fg2"]))]
  Foreground2,
  #[cfg_attr(feature = "cli", clap(aliases = ["f3", "fg3"]))]
  Foreground3,
  #[cfg_attr(feature = "cli", clap(aliases = ["f4", "fg4"]))]
  Foreground4,
  #[cfg_attr(feature = "cli", clap(aliases = ["f5", "fg5"]))]
  Foreground5,
  #[cfg_attr(feature = "cli", clap(aliases = ["f"]))]
  Full,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(ValueEnum))]
pub enum Game {
  #[cfg_attr(feature = "cli", clap(aliases = ["n"]))]
  None,
  #[cfg_attr(feature = "cli", clap(name = "wuwa", alias = "w"))]
  WuWa,
}
