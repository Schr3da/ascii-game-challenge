use std::io::Stdout;

use core_dtos::prelude::{UiView, UiViewIds};
use tui::{backend::CrosstermBackend, Terminal};

use crate::export::prelude::*;

pub fn draw_to_terminal_handler(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    view: &Option<UiView>,
) {
    let next = match view {
        Some(v) => v,
        None => return,
    };

    match next.id {
        UiViewIds::Main => _ = terminal.draw(|f| render_menu(f, next)),
        UiViewIds::Options => _ = terminal.draw(|f| render_options(f, next)),
        UiViewIds::Game => _ = terminal.draw(|f| render_game(f, next)),
    };
}
