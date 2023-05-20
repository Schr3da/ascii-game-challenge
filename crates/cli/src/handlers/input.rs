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
            state
                .send(SendEvents::General(GeneralEvents::OnApplicationWillClose))
                .await
        }
        KeyCode::Char('s') => {
            state
                .send(SendEvents::Renderer(RenderEvents::OnWorldWillUpdate))
                .await;
        }
        KeyCode::Enter => state.send(SendEvents::Ui(UiEvents::OnClick)).await,
        _ => return false,
    };

    true
}
