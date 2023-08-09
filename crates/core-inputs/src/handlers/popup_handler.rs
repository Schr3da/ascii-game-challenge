use core_dtos::prelude::*;
use core_state::prelude::*;

pub async fn handle_popup_event(key: Keys, app_state: &mut AppState) -> bool {
    match key {
        Keys::Backspace => {
            let event = SendEvents::Commands(CommandInputEvents::Pop);
            app_state.send(event).await;
        }
        Keys::Esc => {
            let event = SendEvents::Commands(CommandInputEvents::Cancel);
            app_state.send(event).await;
        }
        Keys::Enter => {
            let event = SendEvents::Commands(CommandInputEvents::Execute(
                app_state.ecs_current_command.clone(),
            ));
            app_state.send(event).await;
        }
        Keys::Char(' ') => {
            let event = SendEvents::Commands(CommandInputEvents::Cancel);
            app_state.send(event).await;
        }
        Keys::DownArrow | Keys::Tab => {
            let event = SendEvents::Ui(UiEvents::OnSelect(SelectionDirections::Next));
            app_state.send(event).await;
        }
        Keys::UpArrow | Keys::BackTab => {
            let event = SendEvents::Ui(UiEvents::OnSelect(SelectionDirections::Previous));
            app_state.send(event).await;
        }
        Keys::Char(s) => {
            let event = SendEvents::Ui(UiEvents::OnShortcut(s.to_string()));
            app_state.send(event).await;
        }
        _ => return false,
    };

    true
}
