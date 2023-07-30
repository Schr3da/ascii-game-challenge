use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Tsify)]
pub enum BuildingIds {
    Lumbarjack,
}

impl ToSelectable for BuildingIds {
    type Item = BuildingIds;

    fn get_selectable_items() -> Vec<Self::Item> {
        vec![BuildingIds::Lumbarjack]
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
                text: "[l] Lumbarjack".to_string(),
            }],
        })]
    }
}
