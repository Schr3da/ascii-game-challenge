use std::io::Stdout;

use tui::{backend::CrosstermBackend, Terminal};

use crate::export::prelude::*;

pub fn draw_to_terminal_handler(terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    match terminal.draw(|f| draw_menu(f)) {
        Err(e) => println!("{:?}", e),
        _ => {}
    };
}
