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
        KeyCode::Char('s') => {
            let event = SendEvents::Renderer(RenderEvents::OnWorldWillUpdate);
            state.send(event).await;
        }
        KeyCode::Enter => {
            let event = SendEvents::Ui(UiEvents::OnClick(ViewComponentIds::Main(MainMenu::Quit)));
            state.send(event).await;
        }
        _ => {}
    };

    true
}
