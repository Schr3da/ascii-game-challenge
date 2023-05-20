use bevy_ecs::prelude::*;

#[derive(Clone, Default, Component)]
pub struct UiViewState {
    pub selected_id: String,
    pub path: Vec<String>,
}
