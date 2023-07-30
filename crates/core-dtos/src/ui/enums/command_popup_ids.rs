use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Tsify)]
pub enum CommandIds {
    Move,
    Build(Option<BuildingIds>),
    Inspect,
    UnknownCommandId,
}

impl Default for CommandIds {
    fn default() -> Self {
        CommandIds::Move
    }
}

impl ToSelectable for CommandIds {
    type Item = ViewComponentIds;

    fn get_selectable_items() -> Vec<ViewComponentIds> {
        vec![
            ViewComponentIds::CommandPopup(CommandIds::Move),
            ViewComponentIds::CommandPopup(CommandIds::Build(None)),
            ViewComponentIds::CommandPopup(CommandIds::Inspect),
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
                    id: ViewComponentIds::CommandPopup(CommandIds::Move),
                    alignment: TextAlignment::Left,
                    text: "[m] Move".to_string(),
                },
                UiLabel {
                    id: ViewComponentIds::CommandPopup(CommandIds::Build(None)),
                    alignment: TextAlignment::Left,
                    text: "[b] Build".to_string(),
                },
                UiLabel {
                    id: ViewComponentIds::CommandPopup(CommandIds::Inspect),
                    alignment: TextAlignment::Left,
                    text: "[i] Inspect".to_string(),
                },
            ],
        })]
    }
}
