use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Tsify)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
