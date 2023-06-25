use crossterm::event::*;

use core_dtos::prelude::*;
use core_state::prelude::*;

pub async fn handle_popup_event(event: KeyEvent, app_state: &mut AppState) -> bool {
    match event.code {
        KeyCode::Backspace => {
            let event = SendEvents::Commands(CommandInputEvents::Pop);
            app_state.send(event).await;
        }
        KeyCode::Esc => {
            let event = SendEvents::Commands(CommandInputEvents::Cancel);
            app_state.send(event).await;
        }
        KeyCode::Enter => {
            let event = SendEvents::Commands(CommandInputEvents::Execute(
                app_state.ecs_current_command.clone(),
            ));
            app_state.send(event).await;
        }
        KeyCode::Char(' ') => {
            let event = SendEvents::Commands(CommandInputEvents::Cancel);
            app_state.send(event).await;
        }
        KeyCode::Down | KeyCode::Tab => {
            let event = SendEvents::Ui(UiEvents::OnSelect(SelectionDirections::Next));
            app_state.send(event).await;
        }
        KeyCode::Up | KeyCode::BackTab => {
            let event = SendEvents::Ui(UiEvents::OnSelect(SelectionDirections::Previous));
            app_state.send(event).await;
        }
        _ => return false,
    };

    true
}
