use crossterm::event;
use tokio::sync::mpsc::error::TryRecvError;

use core_dtos::prelude::*;
use core_inputs::prelude::*;
use core_state::prelude::*;

use crate::export::prelude::*;

pub struct CliInputHandler;

impl CliInputHandler {
    async fn on_key_pressed(event: event::KeyEvent, app_state: &mut AppState) -> bool {
        let key = match event.code {
            event::KeyCode::Up => Keys::UpArrow,
            event::KeyCode::Down => Keys::DownArrow,
            event::KeyCode::Right => Keys::RightArrow,
            event::KeyCode::Left => Keys::LeftArrow,
            event::KeyCode::Enter => Keys::Enter,
            event::KeyCode::Esc => Keys::Esc,
            event::KeyCode::Backspace => Keys::Backspace,
            event::KeyCode::Tab => Keys::Tab,
            event::KeyCode::BackTab => Keys::BackTab,
            event::KeyCode::Char(s) => Keys::Char(s),
            _ => return false,
        };

        return KeyboardInputs::handle_event(key, app_state).await;
    }

    async fn on_mouse_event(event: event::MouseEvent, app_state: &mut AppState) -> bool {
        let next = match event.kind {
            event::MouseEventKind::Moved => {
                MouseEvent::OnMove(event.column as i32, event.row as i32)
            }
            event::MouseEventKind::Up(event::MouseButton::Left) => MouseEvent::OnPress(true),
            _ => return false,
        };

        MouseInputs::handle_event(next, app_state).await
    }

    pub async fn handle(
        event: Result<InpuEvents, TryRecvError>,
        app_state: &mut AppState,
    ) -> bool {
        match event {
            Ok(InpuEvents::Keyboard(e)) => Self::on_key_pressed(e, app_state).await,
            Ok(InpuEvents::Mouse(e)) => Self::on_mouse_event(e, app_state).await,
            Ok(InpuEvents::Window(e)) => WindowInputs::handle_event(e, app_state).await,
            Err(_) => false,
        }
    }
}
