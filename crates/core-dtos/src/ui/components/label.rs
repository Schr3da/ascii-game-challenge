use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Component, Serialize, Deserialize)]
pub struct UiLabel {
    pub id: ViewComponentIds,
    pub text: String,
    pub alignment: TextAlignment,
}
