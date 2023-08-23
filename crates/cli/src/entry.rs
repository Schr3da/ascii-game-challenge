use core_dtos::prelude::*;
use core_state::prelude::*;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io::{stdout, Error, Stdout};
use std::time::Duration;
use tokio::time::sleep;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::export::prelude::*;

pub struct Entry {
    terminal: Option<Terminal<CrosstermBackend<Stdout>>>,
}

impl Default for Entry {
    fn default() -> Self {
        _ = enable_raw_mode();

        let mut stdout = stdout();
        _ = execute!(stdout, EnterAlternateScreen, EnableMouseCapture);

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).ok();

        Self { terminal }
    }
}

impl Entry {
    pub async fn init(&mut self) -> Result<(), Error> {
        let terminal = match &mut self.terminal {
            Some(t) => t,
            None => panic!("Unable to init terminal"),
        };

        let mut state = AppState::default();
        state.subscribe();

        let mut input = InputManager::default();
        input.subscribe();

        let size = terminal.size().unwrap_or_default();

        state
            .send(SendEvents::General(
                GeneralEvents::OnApplicationWillInitialise(size.width, size.height),
            ))
            .await;

        loop {
            let subscription_event = state.ecs_subscription_receiver.try_recv();
            let will_exit = CliEcsHandler::handle(subscription_event, terminal, &mut state).await;
            if !will_exit {
                break;
            }

            let input_event = input.event_receiver.try_recv();
            let did_receive_input = CliInputHandler::handle(input_event, &mut state).await;
            if !did_receive_input {
                sleep(Duration::from_millis(16)).await;
            }
        }

        Ok(())
    }
}

impl Drop for Entry {
    fn drop(&mut self) {
        _ = disable_raw_mode();

        let terminal = match &mut self.terminal {
            Some(t) => t,
            None => return,
        };

        _ = execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );

        _ = terminal.show_cursor();
    }
}
