use bevy_ecs::prelude::*;

use crate::prelude::*;

#[derive(Component)]
pub struct UiList {
    pub id: String,
    pub children: Vec<UiLabel>,
}
