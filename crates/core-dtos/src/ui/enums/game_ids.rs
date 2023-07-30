use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash, Tsify)]
pub enum GameIds {
    Day,
    Time,
    Turns,
    Canvas,
    Menu,
    None,
}

impl ToString for GameIds {
    fn to_string(&self) -> String {
        match self {
            Self::Day => "Day".to_string(),
            Self::Time => "Time".to_string(),
            Self::Turns => "Turns".to_string(),
            Self::Menu => "[ecs] Menu".to_string(),
            Self::Canvas => "".to_string(),
            Self::None => "".to_string(),
        }
    }
}

impl ToSelectable for GameIds {
    type Item = ViewComponentIds;

    fn get_selectable_items() -> Vec<ViewComponentIds> {
        vec![]
    }
}
