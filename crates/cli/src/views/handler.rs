use std::io::Stdout;

use core_dtos::prelude::{UiView, UiViewIds};
use tui::{backend::CrosstermBackend, Terminal};

use crate::export::prelude::*;

pub fn draw_to_terminal_handler(
    view: Option<UiView>,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) {
    let next = match view {
        Some(v) => v,
        None => return,
    };

    match next.id {
        UiViewIds::Main => _ = terminal.draw(|f| draw_menu(f, next)),
        UiViewIds::Options => return,
        UiViewIds::Game => return,
    };
}
