use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tsify::*;

use core_formatters::prelude::*;

use crate::prelude::*;

pub static DEFAULT_DAYS: i32 = 1;

pub static DEFAULT_HOURS: i32 = 8;

#[derive(Debug, Clone, Serialize, Deserialize, Tsify, Eq, PartialEq)]
pub struct GameViewHeaderState {
    #[serde(rename(serialize = "currentDays", deserialize = "currentDays"))]
    pub current_days: String,
    #[serde(rename(serialize = "currentHours", deserialize = "currentHours"))]
    pub current_hours: String,
    #[serde(rename(serialize = "currentMinutes", deserialize = "currentMinutes"))]
    pub current_minutes: String,
    #[serde(rename(serialize = "tickCount", deserialize = "tickCount"))]
    pub tick_count: String,
}

impl Default for GameViewHeaderState {
    fn default() -> Self {
        GameViewHeaderState {
            current_days: DEFAULT_DAYS.to_string(),
            current_hours: prettify_i32(DEFAULT_HOURS),
            current_minutes: prettify_i32(0),
            tick_count: 0.to_string(),
        }
    }
}

impl From<&GameViewHeaderState> for HashMap<ViewComponentIds, String> {
    fn from(value: &GameViewHeaderState) -> HashMap<ViewComponentIds, String> {
        let mut next = HashMap::new();
        next.insert(
            ViewComponentIds::Game(GameIds::Time),
            format!(
                "{} {}:{}",
                GameIds::Time.to_string(),
                value.current_hours,
                value.current_minutes
            ),
        );

        next.insert(
            ViewComponentIds::Game(GameIds::Day),
            format!("{} {}", GameIds::Day.to_string(), value.current_days),
        );

        next.insert(
            ViewComponentIds::Game(GameIds::Turns),
            format!("{} {}", GameIds::Turns.to_string(), value.tick_count),
        );

        next
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Tsify, Eq, PartialEq)]
pub struct GameViewFooterState {}
