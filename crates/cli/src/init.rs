use core_ecs::prelude::*;
use core_state::prelude::*;
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

    let mut assets = AssetResources::default();
    assets.load();

    let mut state = AppState::default();
    state.subscribe();

    let mut input = InputManager::default();
    input.subscribe();

    loop {
        terminal.draw(|f| draw_menu(f))?;
        terminal.draw(|f| draw_game(f, &assets))?;

        match state.subscription_receiver.try_recv() {
            _ => {}
        };

        match input.event_receiver.recv().await {
            Some(KeyCode::Char('q')) => break,
            Some(KeyCode::Char('s')) => {
                state.send(SendEvents::OnWorldWillUpdate).await;
            }
            _ => {}
        };
    }

    Ok(terminal)
}
