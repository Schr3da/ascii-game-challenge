use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Clone, Tsify, Serialize, Deserialize, PartialEq, Eq)]
pub enum SelectedCellNavigation {
    Up,
    Down,
    Left,
    Right,
    Custom(i32, i32),
}
