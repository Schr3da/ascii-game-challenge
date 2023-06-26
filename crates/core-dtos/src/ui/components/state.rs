use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone, Default, Component, Serialize, Deserialize, Tsify)]
pub struct UiViewState {
    pub selected_id: ViewComponentIds,
    pub selectable_ids: Vec<ViewComponentIds>,
    pub view_data: ViewDataTypes,
}

impl UiViewState {
    pub fn has_quick_action_data(&self) -> bool {
        match self.view_data {
            ViewDataTypes::QuickActionData => true,
            ViewDataTypes::NoViewData => false,
        }
    }
}
