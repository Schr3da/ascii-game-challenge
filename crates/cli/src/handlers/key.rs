use crossterm::event::*;

use core_dtos::prelude::*;
use core_state::prelude::*;
use tokio::sync::mpsc::error::TryRecvError;

pub async fn key_pressed_handler(event: Result<KeyCode, TryRecvError>, state: &mut AppState) {
    let next = match event {
        Ok(e) => e,
        Err(_) => return,
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
        _ => {}
    };
}
