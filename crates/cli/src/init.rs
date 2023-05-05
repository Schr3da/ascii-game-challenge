use crossterm::event::{EnableMouseCapture, KeyCode};
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use std::io::{stdout, Error, Stdout};
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::export::prelude::*;

pub async fn terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Error> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let state = AppState::default();

    let mut manager = InputManager::default();
    manager.register();

    loop {
        terminal.draw(|f| draw_menu(f, &state))?;

        let key = match manager.event_receiver.recv().await {
            Some(e) => e,
            _ => continue,
        };

        match key {
            KeyCode::Char('q') => break,
            _ => {}
        }
    }

    Ok(terminal)
}
