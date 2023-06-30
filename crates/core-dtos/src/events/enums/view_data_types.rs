use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone, Default, Serialize, Deserialize, Tsify)]
pub enum ViewDataTypes {
    #[default]
    NoViewData,
    QuickActionData,
    GameHeader(GameViewHeaderState),
}

impl From<&ViewDataTypes> for HashMap<ViewComponentIds, i32> {
    fn from(value: &ViewDataTypes) -> Self {
        match value {
            ViewDataTypes::NoViewData => HashMap::new(),
            ViewDataTypes::QuickActionData => HashMap::new(),
            ViewDataTypes::GameHeader(s) => {
                let mut next = HashMap::new();
                next.insert(ViewComponentIds::Game(GameIds::Time), s.current_time);
                next.insert(ViewComponentIds::Game(GameIds::Turns), s.tick_count);
                next
            }
        }
    }
}
