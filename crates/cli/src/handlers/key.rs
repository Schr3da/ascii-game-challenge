use crossterm::event::*;

use core_dtos::prelude::*;
use core_state::prelude::*;

pub async fn key_pressed_handler(event: Option<KeyCode>, state: &mut AppState) {
    match event {
        Some(KeyCode::Char('q')) => {
            state
                .send(SendEvents::General(GeneralEvents::OnApplicationWillClose))
                .await
        }
        Some(KeyCode::Char('s')) => {
            state
                .send(SendEvents::Renderer(RenderEvents::OnWorldWillUpdate))
                .await;
        }
        Some(KeyCode::Enter) => state.send(SendEvents::Ui(UiEvents::OnClick)).await,
        _ => {}
    };
}
