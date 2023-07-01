use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use core_dtos::prelude::*;
use core_state::prelude::*;

async fn handle_did_move(event: MouseEvent, app_state: &mut AppState) {
    app_state
        .send(SendEvents::Renderer(RenderEvents::OnUpdateSelectedCell(
            SelectedCellNavigation::Custom(event.column as i32, event.row as i32),
        )))
        .await;
}

async fn handle_did_select(button: MouseButton, app_state: &mut AppState) {
    if button != MouseButton::Left {
        return;
    }

    let event = SendEvents::Commands(CommandInputEvents::New);
    app_state.send(event).await;
}

pub async fn handle_mouse_view_event(event: MouseEvent, app_state: &mut AppState) -> bool {
    match event.kind {
        MouseEventKind::Down(b) => handle_did_select(b, app_state).await,
        MouseEventKind::Moved => handle_did_move(event, app_state).await,
        _ => return false,
    };

    true
}
