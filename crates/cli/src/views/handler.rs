use std::io::Stdout;

use core_ecs::prelude::AssetResources;
use tui::{backend::CrosstermBackend, Terminal};

use crate::export::prelude::*;

pub fn draw_to_terminal_handler(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    assets: &AssetResources,
) {
    _ = terminal.draw(|f| draw_menu(f));
    _ = terminal.draw(|f| draw_game(f, assets));
}
