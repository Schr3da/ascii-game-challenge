use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Tsify)]
pub struct Cell {
    pub symbol: Ascii,
    pub background: CellColors,
    pub foreground: CellColors,
    #[serde(rename = "isBold")]
    pub is_bold: bool,
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
