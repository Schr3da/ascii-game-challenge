use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Eq, PartialEq, Clone, Tsify, Serialize, Deserialize)]
pub enum CameraNavigation {
    Left,
    Right,
    Up,
    Down,
}