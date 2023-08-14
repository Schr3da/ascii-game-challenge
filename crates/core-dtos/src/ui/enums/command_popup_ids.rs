use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Tsify)]
pub enum CommandIds {
    Build(Option<BuildingIds>),
    UnknownCommandId,
}

impl Default for CommandIds {
    fn default() -> Self {
        CommandIds::Build(None)
    }
}

impl ToShortcut for CommandIds {
    fn get_shortcut(&self) -> Option<String> {
        match self {
            Self::Build(_) => Some("b".to_string()),
            Self::UnknownCommandId => None,
        }
    }
}

impl ToSelectable for CommandIds {
    type Item = ViewComponentIds;

    fn get_selectable_items() -> Vec<ViewComponentIds> {
        vec![
            ViewComponentIds::CommandPopup(CommandIds::Build(None)),
        ]
    }
}

impl ToUiViewChildren for CommandIds {
    fn get_ui_items() -> Vec<UiViewChild> {
        vec![UiViewChild::List(UiList {
            id: ViewComponentIds::Main(MainMenuIds::MenuList),
            label: "Available Actions".to_string(),
            children: vec![
                UiLabel {
                    id: ViewComponentIds::CommandPopup(CommandIds::Build(None)),
                    alignment: TextAlignment::Left,
                    text: "Build".to_string(),
                    shortcut: CommandIds::Build(None).get_shortcut(),
                },
            ],
        })]
    }
}
