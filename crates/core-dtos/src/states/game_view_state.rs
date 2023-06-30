use tsify::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, Tsify, Eq, PartialEq)]
pub struct GameViewHeaderState {
  #[serde(rename(serialize = "tickCount", deserialize = "tickCount"))]
  pub tick_count: i32,
  #[serde(rename(serialize = "currentTime", deserialize = "currentTime"))]
  pub current_time: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Tsify, Eq, PartialEq)]
pub struct GameViewFooterState {
}