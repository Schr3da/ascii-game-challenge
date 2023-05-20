use bevy_ecs::prelude::*;

#[derive(Clone, Component)]
pub struct UiLabel {
    pub id: String,
    pub text: String,
}
