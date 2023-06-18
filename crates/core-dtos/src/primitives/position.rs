use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Tsify)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
