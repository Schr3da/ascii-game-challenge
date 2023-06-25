use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::{Cell, Rect};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Tsify)]
pub struct SelectedCell {
    pub top: u16,
    pub bottom: u16,
    pub frame: Rect,
    pub cell: Cell,
}

impl Default for SelectedCell {
    fn default() -> Self {
        SelectedCell {
            top: 0,
            bottom: 0,
            cell: Cell::default(),
            frame: Rect {
                x: 0,
                y: 0,
                width: 1,
                height: 1,
            },
        }
    }
}
