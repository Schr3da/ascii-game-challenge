use crossterm::event::*;

use core_dtos::prelude::*;
use core_state::prelude::*;

async fn handle_command_popup_event(event: KeyEvent, app_state: &mut AppState) -> bool {
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

async fn handle_quick_action_popup_event(event: KeyEvent, app_state: &mut AppState) -> bool {
    match event.code {
        KeyCode::Esc => {
            let event = SendEvents::QuickAction(QuickActionEvents::Cancel);
            app_state.send(event).await;
        }
        KeyCode::Enter => {
            let event = SendEvents::QuickAction(QuickActionEvents::Execute);
            app_state.send(event).await;
        }
        _ => return false,
    };

    true
}

pub async fn handle_popup_event(
    event: KeyEvent,
    app_state: &mut AppState,
    view_state: &UiViewState,
) -> bool {
    if view_state.has_quick_action_data() {
        return handle_quick_action_popup_event(event, app_state).await;
    }

    return handle_command_popup_event(event, app_state).await;
}
