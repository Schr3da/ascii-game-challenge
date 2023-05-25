use bevy_ecs::prelude::*;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Component)]
pub struct UiList {
    pub id: ViewComponentIds,
    pub label: String,
    pub children: Vec<UiLabel>,
}
