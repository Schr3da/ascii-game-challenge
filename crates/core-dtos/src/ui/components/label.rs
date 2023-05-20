use bevy_ecs::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct UiLabel {
    pub id: String,
    pub text: String,
}
