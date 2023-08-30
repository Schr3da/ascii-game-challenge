use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, Tsify, Serialize, Deserialize, PartialEq, Eq)]
pub enum Navigation {
    Up,
    Down,
    Left,
    Right,
    Custom(i32, i32),
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum SelectionDirections {
    Next,
    Previous,
}
