use core_ecs::prelude::*;
use tui::style::Color;

pub fn to_terminal_color(next: &CellColors) -> Color {
    match next {
        CellColors::Black => Color::Black,
        CellColors::White => Color::White,
    }
}
