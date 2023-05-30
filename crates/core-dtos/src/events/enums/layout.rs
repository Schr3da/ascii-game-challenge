use tsify::Tsify;

use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Tsify)]
pub enum LayoutAlignments {
    Horizontal,
    Vertical,
}

impl Default for LayoutAlignments {
    fn default() -> Self {
        LayoutAlignments::Vertical
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Tsify)]
pub enum LayoutConstraints {
    Percentage(u16),
    Value(u16),
    MinValue(u16),
    MaxValue(u16),
}

impl Default for LayoutConstraints {
    fn default() -> Self {
        LayoutConstraints::Percentage(100)
    }
}
