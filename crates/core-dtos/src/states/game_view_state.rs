use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tsify::*;

use crate::prelude::*;

pub static DEFAULT_DAYS: i32 = 1;

pub static DEFAULT_HOURS: i32 = 8;

#[derive(Debug, Clone, Serialize, Deserialize, Tsify, Eq, PartialEq)]
pub struct GameViewHeaderState {
    #[serde(rename(serialize = "currentDays", deserialize = "currentDays"))]
    pub current_days: i32,
    #[serde(rename(serialize = "currentHours", deserialize = "currentHours"))]
    pub current_hours: i32,
    #[serde(rename(serialize = "currentMinutes", deserialize = "currentMinutes"))]
    pub current_minutes: i32,
    #[serde(rename(serialize = "tickCount", deserialize = "tickCount"))]
    pub tick_count: i32,
}

impl Default for GameViewHeaderState {
    fn default() -> Self {
        GameViewHeaderState {
            current_days: DEFAULT_DAYS,
            current_hours: DEFAULT_HOURS,
            current_minutes: 0,
            tick_count: 0,
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
