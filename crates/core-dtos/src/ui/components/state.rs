use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::ViewComponentIds;

#[derive(Debug, Eq, PartialEq, Clone, Default, Component, Serialize, Deserialize)]
pub struct UiViewState {
    pub selected_id: ViewComponentIds,
    pub selectable_ids: Vec<ViewComponentIds>,
}
