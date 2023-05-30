use bevy_ecs::prelude::*;

use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Component, Tsify, Serialize, Deserialize)]
pub struct Interaction {
    pub is_enabled: bool,
    pub is_selected: bool,
}
