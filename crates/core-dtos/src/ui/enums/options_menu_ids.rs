use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash, Tsify)]
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

impl ToUiViewChildren for OptionMenuIds {
    fn get_ui_items() -> Vec<UiViewChild> {
        vec![
            UiViewChild::Label(UiLabel {
                id: ViewComponentIds::Options(OptionMenuIds::Title),
                text: "".to_string(),
                alignment: TextAlignment::Center,
            }),
            UiViewChild::List(UiList {
                id: ViewComponentIds::Options(OptionMenuIds::OptionList),
                label: "Options".to_string(),
                children: vec![
                    UiLabel {
                        id: ViewComponentIds::Options(OptionMenuIds::LevelOfDifficulty),
                        text: "Level".to_string(),
                        alignment: TextAlignment::Left,
                    },
                    UiLabel {
                        id: ViewComponentIds::Options(OptionMenuIds::Sound),
                        text: "Sound".to_string(),
                        alignment: TextAlignment::Left,
                    },
                    UiLabel {
                        id: ViewComponentIds::Options(OptionMenuIds::Back),
                        text: "Back".to_string(),
                        alignment: TextAlignment::Left,
                    },
                ],
            }),
        ]
    }
}
