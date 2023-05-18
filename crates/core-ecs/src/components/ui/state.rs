use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct UiState {
    pub selected_id: String,
    pub path: String,
}
