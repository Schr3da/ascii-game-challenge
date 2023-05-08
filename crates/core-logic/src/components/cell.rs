use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Cell {
    pub symbol: Ascii,
    pub background: CellColors,
    pub foreground: CellColors,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            symbol: Ascii::Space,
            background: CellColors::Black,
            foreground: CellColors::White,
        }
    }
}
