use crossterm::event::*;

use core_dtos::prelude::*;
use core_state::prelude::*;
use tokio::sync::mpsc::error::TryRecvError;

pub async fn input_handler(event: Result<KeyCode, TryRecvError>, app_state: &mut AppState) -> bool {
    let next = match event {
        Ok(e) => e,
        Err(_) => return false,
    };

    match next {
        KeyCode::Tab => {
            let event = SendEvents::Ui(UiEvents::OnSelect(SelectionDirections::Next));
            app_state.send(event).await;
        }
        KeyCode::BackTab => {
            let event = SendEvents::Ui(UiEvents::OnSelect(SelectionDirections::Previous));
            app_state.send(event).await;
        }
        KeyCode::Enter => {
            if let Some(s) = &app_state.ecs_current_view_state {
                let event = SendEvents::Ui(UiEvents::OnClick(s.selected_id.clone()));
                app_state.send(event).await;
            };
        }
        _ => {}
    };

    true
}
