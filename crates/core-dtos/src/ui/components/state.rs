use bevy_ecs::prelude::*;

use crate::prelude::ViewComponentIds;

#[derive(Debug, Clone, Default, Component)]
pub struct UiViewState {
    pub selected_id: ViewComponentIds,
}
