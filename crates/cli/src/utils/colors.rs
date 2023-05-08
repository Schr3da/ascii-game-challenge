use core_logic::prelude::*;
use tui::style::Color;

pub fn _to_terminal_color(next: CellColors) -> Color {
    match next {
        CellColors::Black => Color::Black,
        CellColors::White => Color::White,
    }
}
