use core_dtos::prelude::*;
use tui::style::Color;

pub fn to_terminal_color(next: &CellColors) -> Color {
    match next {
        CellColors::Rgb(r, g, b) => Color::Rgb(*r, *g, *b),
    }
}
