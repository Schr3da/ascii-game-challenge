use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Component, Serialize, Deserialize)]
pub struct UiList {
    pub id: ViewComponentIds,
    pub label: String,
    pub children: Vec<UiLabel>,
}
