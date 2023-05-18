use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Interaction {
    pub is_enabled: bool,
    pub is_selected: bool,
}
