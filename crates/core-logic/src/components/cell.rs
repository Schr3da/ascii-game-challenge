use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Cell {
    pub is_bold: bool,
    pub symbol: Ascii,
    pub background: CellColors,
    pub foreground: CellColors,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            is_bold: false,
            symbol: Ascii::Space,
            background: CellColors::Black,
            foreground: CellColors::White,
        }
    }
}
