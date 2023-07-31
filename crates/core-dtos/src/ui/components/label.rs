use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Component, Serialize, Deserialize, Tsify)]
pub struct UiLabel {
    pub id: ViewComponentIds,
    pub text: String,
    pub alignment: TextAlignment,
    pub shortcut: Option<String>,
}
