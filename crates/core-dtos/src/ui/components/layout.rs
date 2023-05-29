use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Default, Debug, Eq, PartialEq, Clone, Component, Serialize, Deserialize)]
pub struct UiLayout {
    pub margin: u16,
    pub alignment: LayoutAlignments,
    pub constraints: Vec<LayoutConstraints>,
}
