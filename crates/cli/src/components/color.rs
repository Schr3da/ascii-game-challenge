use crossterm::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum CellColors {
    Black,
    White,
}

impl Default for CellColors {
    fn default() -> Self {
        CellColors::Black
    }
}

impl Into<Color> for CellColors {
    fn into(self) -> Color {
        match self {
            CellColors::Black => Color::Black,
            CellColors::White => Color::White,
        }
    }
}
