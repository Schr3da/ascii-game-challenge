use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Tsify)]
pub enum BuildingIds {
    Lumbarjack,
}

impl ToShortcut for BuildingIds {
    fn get_shortcut(&self) -> Option<String> {
        match self {
            BuildingIds::Lumbarjack => Some("l".to_string()),
        }
    }
}

impl ToSelectable for BuildingIds {
    type Item = ViewComponentIds;

    fn get_selectable_items() -> Vec<Self::Item> {
        vec![ViewComponentIds::CommandPopup(CommandIds::Build(Some(
            BuildingIds::Lumbarjack,
        )))]
    }
}

impl ToUiViewChildren for BuildingIds {
    fn get_ui_items() -> Vec<UiViewChild> {
        vec![UiViewChild::List(UiList {
            id: ViewComponentIds::CommandPopup(CommandIds::Build(None)),
            label: "Available Buildings".to_string(),
            children: vec![UiLabel {
                id: ViewComponentIds::CommandPopup(CommandIds::Build(Some(
                    BuildingIds::Lumbarjack,
                ))),
                alignment: TextAlignment::Left,
                text: "Lumbarjack".to_string(),
                shortcut: BuildingIds::Lumbarjack.get_shortcut(),
            }],
        })]
    }
}
