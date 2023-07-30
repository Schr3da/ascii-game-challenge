use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash, Tsify)]
pub enum MainMenuIds {
    Title,
    MenuList,
    NewGame,
    Options,
    Quit,
}

impl ToString for MainMenuIds {
    fn to_string(&self) -> String {
        match self {
            Self::Title => "Ascii Game Challenge".to_string(),
            Self::MenuList => "Main Menu".to_string(),
            Self::NewGame => "New Game".to_string(),
            Self::Options => "Options".to_string(),
            Self::Quit => "Quit".to_string(),
        }
    }
}

impl ToSelectable for MainMenuIds {
    type Item = ViewComponentIds;

    fn get_selectable_items() -> Vec<ViewComponentIds> {
        vec![
            ViewComponentIds::Main(MainMenuIds::NewGame),
            ViewComponentIds::Main(MainMenuIds::Options),
            ViewComponentIds::Main(MainMenuIds::Quit),
        ]
    }
}

impl ToUiViewChildren for MainMenuIds {
    fn get_ui_items() -> Vec<UiViewChild> {
        vec![
            UiViewChild::Label(UiLabel {
                id: ViewComponentIds::Main(MainMenuIds::Title),
                text: "Ascii game challenge".to_string(),
                alignment: TextAlignment::Center,
            }),
            UiViewChild::List(UiList {
                id: ViewComponentIds::Main(MainMenuIds::MenuList),
                label: "Main Menu".to_string(),
                children: vec![
                    UiLabel {
                        id: ViewComponentIds::Main(MainMenuIds::NewGame),
                        text: "New Game".to_string(),
                        alignment: TextAlignment::Left,
                    },
                    UiLabel {
                        id: ViewComponentIds::Main(MainMenuIds::Options),
                        text: "Options".to_string(),
                        alignment: TextAlignment::Left,
                    },
                    UiLabel {
                        id: ViewComponentIds::Main(MainMenuIds::Quit),
                        text: "Quit".to_string(),
                        alignment: TextAlignment::Left,
                    },
                ],
            }),
        ]
    }
}
