use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct UiLabel {
    pub id: String,
    pub text: String,
}
