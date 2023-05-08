
use tui::style::Color;
use core_logic::prelude::*;

pub struct TerminalColor(Color);


macro_rules! generate_color_mapping {
    () => {

        fn to_terminal_color() -> Color {
            /*
            match self {
                CellColors::Black => Color::Black,
                CellColors::White => Color::White,
            }
            */

            Color::Black
        }

    };
}

generate_color_mapping!();
