use crossterm::event::*;

use core_dtos::prelude::*;
use core_state::prelude::*;
use tokio::sync::mpsc::error::TryRecvError;

pub async fn input_handler(event: Result<KeyCode, TryRecvError>, state: &mut AppState) -> bool {
    let next = match event {
        Ok(e) => e,
        Err(_) => return false,
    };

    match next {
        KeyCode::Char('q') => {
            let next_state_event = SendEvents::General(GeneralEvents::OnApplicationWillClose);
            state.send(next_state_event).await;
        }
        KeyCode::Char('s') => {
            let next_state_event = SendEvents::Renderer(RenderEvents::OnWorldWillUpdate);
            state.send(next_state_event).await;
        }
        KeyCode::Enter => {
            let next_state_event = SendEvents::Ui(UiEvents::OnClick);
            state.send(next_state_event).await;
        }
        _ => {}
    };

    true
}
