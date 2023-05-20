use core_dtos::prelude::*;
use core_state::prelude::*;
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use std::io::{stdout, Error, Stdout};
use std::time::Duration;
use tokio::time::sleep;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::export::prelude::*;

pub async fn terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Error> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

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
        let subscription_event = state.ecs_subscription_receiver.try_recv();
        let will_exit = subscription_handler(subscription_event, &mut terminal, &mut state).await;
        if !will_exit {
            break;
        }

        let input_event = input.event_receiver.try_recv();
        let did_receive_input = input_handler(input_event, &mut state).await;
        if !did_receive_input {
            sleep(Duration::from_millis(16)).await;
        }
    }

    Ok(terminal)
}
