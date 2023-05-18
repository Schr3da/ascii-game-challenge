use core_dtos::prelude::*;
use core_ecs::prelude::*;
use core_state::prelude::*;
use crossterm::event::EnableMouseCapture;
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

    state
        .send(SendEvents::General(
            GeneralEvents::OnApplicationWillInitialise,
        ))
        .await;

    loop {
        let subscription_event = state.ecs_subscription_receiver.recv().await;
        match subscription_handler(subscription_event) {
            true => draw_to_terminal_handler(&mut terminal, &assets),
            false => break,
        }

        let input_event = input.event_receiver.recv().await;
        key_pressed_handler(input_event, &mut state).await;
    }

    Ok(terminal)
}
