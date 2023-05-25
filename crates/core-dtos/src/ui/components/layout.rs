use bevy_ecs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone, Default, Component)]
pub struct UiLayout {
    pub margin: u16,
    pub alignment: LayoutAlignments,
    pub constraints: Vec<LayoutConstraints>,
}
