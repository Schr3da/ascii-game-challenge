use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Tsify)]
pub enum OptionMenuIds {
    Title,
    OptionList,
    LevelOfDifficulty,
    Sound,
    Back,
}

impl ToString for OptionMenuIds {
    fn to_string(&self) -> String {
        match self {
            Self::Title => "Options".to_string(),
            Self::OptionList => "Game Options".to_string(),
            Self::Sound => "Sound".to_string(),
            Self::LevelOfDifficulty => "Level".to_string(),
            Self::Back => "Back".to_string(),
        }
    }
}

impl ToSelectable for OptionMenuIds {
    type Item = ViewComponentIds;

    fn get_selectable_items() -> Vec<ViewComponentIds> {
        vec![
            ViewComponentIds::Options(OptionMenuIds::LevelOfDifficulty),
            ViewComponentIds::Options(OptionMenuIds::Sound),
            ViewComponentIds::Options(OptionMenuIds::Back),
        ]
    }
}
