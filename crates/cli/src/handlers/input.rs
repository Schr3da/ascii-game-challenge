use crossterm::event::*;

use core_dtos::prelude::*;
use core_state::prelude::*;
use tokio::sync::mpsc::error::TryRecvError;

use crate::export::prelude::*;

async fn handle_view_event(event: KeyEvent, app_state: &mut AppState) -> bool {
    match event.code {
        KeyCode::Esc => {
            let event = SendEvents::Ui(UiEvents::OnCloseView);
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
        KeyCode::Enter => {
            if let Some(s) = &app_state.ecs_current_view_state {
                let event = SendEvents::Ui(UiEvents::OnClick(s.selected_id.clone()));
                app_state.send(event).await;
            };
        }
        KeyCode::Char('q') => {
            let event =
                SendEvents::Ui(UiEvents::OnClick(ViewComponentIds::Main(MainMenuIds::Quit)));
            app_state.send(event).await;
        }
        KeyCode::Char(' ') => {
            let event = SendEvents::Commands(CommandInputEvents::New);
            app_state.send(event).await;
        }
        _ => return false,
    };

    true
}

async fn handle_popup_event(event: KeyEvent, app_state: &mut AppState) -> bool {
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

async fn handle_keyboard_event(event: KeyEvent, app_state: &mut AppState) -> bool {
    match &app_state.ecs_current_popup_state {
        Some(_) => handle_popup_event(event, app_state).await,
        None => handle_view_event(event, app_state).await,
    }
}

async fn handle_mouse_event(event: MouseEvent, _app_state: &mut AppState) -> bool {
    let _x = event.column;
    let _y = event.row;

    true
}

async fn handle_window_event(event: WindowEvents, app_state: &mut AppState) -> bool {
    match event {
        WindowEvents::Resize(w, h) => {
            app_state
                .send(SendEvents::General(GeneralEvents::OnApplicationResize(
                    w, h,
                )))
                .await
        }
    };

    true
}

pub async fn input_handler(
    event: Result<InpuEvents, TryRecvError>,
    app_state: &mut AppState,
) -> bool {
    match event {
        Ok(InpuEvents::Keyboard(e)) => handle_keyboard_event(e, app_state).await,
        Ok(InpuEvents::Mouse(e)) => handle_mouse_event(e, app_state).await,
        Ok(InpuEvents::Window(e)) => handle_window_event(e, app_state).await,
        Err(_) => false,
    }
}
