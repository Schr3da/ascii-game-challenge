use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
