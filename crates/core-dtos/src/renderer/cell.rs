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

impl From<&SelectedCell> for ViewDataTypes {
    fn from(value: &SelectedCell) -> Self {
        ViewDataTypes::Popup(PopupState {
            current_selected_game_tile: value.clone(),
        })
    }
}

impl SelectedCell {
    pub fn handle_window_resize(&mut self, width: i32, height: i32) {
        if self.frame.x >= width {
            self.frame.x = width - self.frame.width;
        }

        if self.frame.y >= height {
            self.frame.y = height - self.bottom as i32 - self.top as i32 - self.frame.height;
        }
    }
}
