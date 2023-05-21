use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct UiStore {
    pub current_view: UiViewIds,
}
