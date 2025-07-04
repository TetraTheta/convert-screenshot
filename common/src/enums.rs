#[cfg(feature = "cli")]
use clap::{Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[serde(rename_all = "lowercase")]
pub enum CropPosition {
  Bottom,
  Center,
  Full,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(Subcommand, ValueEnum))]
pub enum Operation {
  /// Process everything [alias: a]
  #[cfg_attr(feature = "cli", clap(aliases = ["a"]))]
  All,
  /// Background [alias: b, bg]
  #[cfg_attr(feature = "cli", clap(aliases = ["b", "bg"]))]
  Background,
  /// Center [alias: c]
  #[cfg_attr(feature = "cli", clap(aliases = ["c"]))]
  Center,
  /// Create Directories [alias: cd]
  #[cfg_attr(feature = "cli", clap(aliases = ["cd"]))]
  CreateDirectory,
  /// Cutscene [alias: cs, s]
  #[cfg_attr(feature = "cli", clap(aliases = ["cs", "s"]))]
  Cutscene,
  /// Foreground with 0 option [alias: f0, fg0]
  #[cfg_attr(feature = "cli", clap(aliases = ["f0", "fg0"]))]
  Foreground0,
  /// Foreground with 1 option [alias: f1, fg1]
  #[cfg_attr(feature = "cli", clap(aliases = ["f1", "fg1"]))]
  Foreground1,
  /// Foreground with 2 options [alias: f2, fg2]
  #[cfg_attr(feature = "cli", clap(aliases = ["f2", "fg2"]))]
  Foreground2,
  /// Foreground with 3 options [alias: f3, fg3]
  #[cfg_attr(feature = "cli", clap(aliases = ["f3", "fg3"]))]
  Foreground3,
  /// Foreground with 4 options [alias: f4, fg4]
  #[cfg_attr(feature = "cli", clap(aliases = ["f4", "fg4"]))]
  Foreground4,
  /// Foreground with 5 options [alias: f5, fg5]
  #[cfg_attr(feature = "cli", clap(aliases = ["f5", "fg5"]))]
  Foreground5,
  /// Fullscreen [alias: f]
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
