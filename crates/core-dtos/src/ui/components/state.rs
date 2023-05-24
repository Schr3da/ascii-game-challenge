use bevy_ecs::prelude::*;

use crate::prelude::ViewComponentIds;

#[derive(Debug, Eq, PartialEq, Clone, Default, Component)]
pub struct UiViewState {
    pub selected_id: ViewComponentIds,
    pub selectable_ids: Vec<ViewComponentIds>,
}
