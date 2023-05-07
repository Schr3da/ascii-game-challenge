use serde::{Deserialize, Serialize};

use super::prelude::CellColors;

#[derive(Clone, Serialize, Deserialize)]
pub struct Cell {
    pub symbol: String,
    pub background: CellColors,
    pub foreground: CellColors,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            symbol: " ".to_string(),
            background: CellColors::Black,
            foreground: CellColors::White,
        }
    }
}
