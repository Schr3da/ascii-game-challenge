use bevy_ecs::prelude::*;

use crate::prelude::*;

#[derive(Default, Debug, Eq, PartialEq, Clone, Component)]
pub struct UiLayout {
    pub margin: u16,
    pub alignment: LayoutAlignments,
    pub constraints: Vec<LayoutConstraints>,
}
