use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Tsify)]
pub enum PopupIds {
    Logger(Option<Vec<String>>),
    Build(Option<BuildingIds>),
    UnknownCommandId,
}

impl Default for PopupIds {
    fn default() -> Self {
        PopupIds::Build(None)
    }
}

impl ToShortcut for PopupIds {
    fn get_shortcut(&self) -> Option<String> {
        match self {
            Self::Build(_) => Some("b".to_string()),
            Self::Logger(_) => Some("l".to_string()),
            Self::UnknownCommandId => None,
        }
    }
}

impl ToSelectable for PopupIds {
    type Item = ViewComponentIds;

    fn get_selectable_items() -> Vec<ViewComponentIds> {
        vec![
            ViewComponentIds::Popup(PopupIds::Build(None)),
            ViewComponentIds::Popup(PopupIds::Logger(None))
        ]
    }
}

impl ToUiViewChildren for PopupIds {
    fn get_ui_items() -> Vec<UiViewChild> {
        vec![UiViewChild::List(UiList {
            id: ViewComponentIds::Main(MainMenuIds::MenuList),
            label: "Available Actions".to_string(),
            children: vec![
                UiLabel {
                    id: ViewComponentIds::Popup(PopupIds::Build(None)),
                    alignment: TextAlignment::Left,
                    text: "Build".to_string(),
                    shortcut: PopupIds::Build(None).get_shortcut(),
                },
                UiLabel {
                    id: ViewComponentIds::Popup(PopupIds::Logger(None)),
                    alignment: TextAlignment::Left,
                    text: "Logger".to_string(),
                    shortcut: PopupIds::Logger(None).get_shortcut(),
                },
            ],
        })]
    }
}
