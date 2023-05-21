use bevy_ecs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct UiLabel {
    pub id: ViewComponentIds,
    pub text: String,
}
