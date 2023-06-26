use crossterm::event::*;
use tokio::sync::mpsc::error::TryRecvError;

use core_dtos::prelude::*;
use core_state::prelude::*;

use crate::export::prelude::*;

async fn handle_keyboard_event(event: KeyEvent, app_state: &mut AppState) -> bool {
    match &app_state.ecs_current_popup_state.clone() {
        Some(view_state) => handle_popup_event(event, app_state, view_state).await,
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
