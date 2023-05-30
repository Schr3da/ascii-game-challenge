use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Component, Serialize, Deserialize, Tsify)]
pub struct UiList {
    pub id: ViewComponentIds,
    pub label: String,
    pub children: Vec<UiLabel>,
}
