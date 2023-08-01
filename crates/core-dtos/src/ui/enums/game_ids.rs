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
    Actions,
    None,
}

impl ToString for GameIds {
    fn to_string(&self) -> String {
        match self {
            Self::Day => "Day".to_string(),
            Self::Time => "Time".to_string(),
            Self::Turns => "Turns".to_string(),
            Self::Menu => "Menu".to_string(),
            Self::Actions => "Actions".to_string(),
            Self::Canvas => "".to_string(),
            Self::None => "".to_string(),
        }
    }
}

impl ToShortcut for GameIds {
    fn get_shortcut(&self) -> Option<String> {
        match self {
            Self::Menu => Some("ecs".to_string()),
            Self::Actions => Some("space".to_string()),
            _ => None,
        }
    }
}

impl ToSelectable for GameIds {
    type Item = ViewComponentIds;

    fn get_selectable_items() -> Vec<ViewComponentIds> {
        vec![]
    }
}
