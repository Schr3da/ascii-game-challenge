use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Tsify)]
pub enum TextAlignment {
    Center,
    Left,
    Right,
}

impl Default for TextAlignment {
    fn default() -> Self {
        TextAlignment::Center
    }
}
