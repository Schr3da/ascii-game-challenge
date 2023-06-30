use std::collections::HashMap;

use tsify::*;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize, Tsify, Eq, PartialEq)]
pub struct GameViewHeaderState {
  #[serde(rename(serialize = "tickCount", deserialize = "tickCount"))]
  pub tick_count: i32,
  #[serde(rename(serialize = "currentTime", deserialize = "currentTime"))]
  pub current_time: i32,
}

impl From<&GameViewHeaderState> for HashMap<ViewComponentIds, i32> {
  fn from(value: &GameViewHeaderState) -> HashMap<ViewComponentIds, i32> {
    let mut next = HashMap::new();
    next.insert(ViewComponentIds::Game(GameIds::Time), value.current_time);
    next.insert(ViewComponentIds::Game(GameIds::Turns), value.tick_count);
    next
  }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Tsify, Eq, PartialEq)]
pub struct GameViewFooterState {
}