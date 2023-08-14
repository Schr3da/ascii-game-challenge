use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Tsify)]
pub struct Cell {
    pub id: AsciiIds,
    pub symbol: Ascii,
    pub background: CellColors,
    pub foreground: CellColors,
    #[serde(rename = "isBold")]
    pub is_bold: bool,
    #[serde(rename = "shortDescription")]
    pub short_description: Option<String>,
    #[serde(rename = "longDescription")]
    pub long_description: Option<String>,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            id: AsciiIds::UnknownAsciiId,
            is_bold: false,
            symbol: Ascii::Space,
            background: CellColors::default(),
            foreground: CellColors::default(),
            short_description: None,
            long_description: None,
        }
    }
}
