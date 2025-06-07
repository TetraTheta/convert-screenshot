use clap::ValueEnum;
use serde::Serialize;
use clap;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize)]
pub enum CropPosition {
  Bottom,
  Center,
  Full,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize)]
pub enum Game {
  #[clap(aliases = ["n"])]
  None,
  #[clap(name = "wuwa", alias = "w")]
  WuWa,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Debug)]
pub enum Operation {
  #[clap(aliases = ["a"])]
  All,
  #[clap(aliases = ["b", "bg"])]
  Background,
  #[clap(aliases = ["c"])]
  Center,
  #[clap(aliases = ["cd"])]
  CreateDirectory,
  #[clap(aliases = ["f0", "fg0"])]
  Foreground0,
  #[clap(aliases = ["f1", "fg1"])]
  Foreground1,
  #[clap(aliases = ["f2", "fg2"])]
  Foreground2,
  #[clap(aliases = ["f3", "fg3"])]
  Foreground3,
  #[clap(aliases = ["f4", "fg4"])]
  Foreground4,
  #[clap(aliases = ["f"])]
  Full,
}
