use core_dtos::prelude::*;
use core_state::prelude::*;

use crate::prelude::MouseEvent;

async fn handle_did_move(column: i32, row: i32, app_state: &mut AppState) {
    app_state
        .send(SendEvents::Renderer(RenderEvents::OnUpdateSelectedCell(
            SelectedCellNavigation::Custom(column, row),
        )))
        .await;
}

async fn handle_did_select(is_primary_button: bool, app_state: &mut AppState) {
    if is_primary_button == false {
        return;
    }

    let event = SendEvents::Commands(CommandInputEvents::New);
    app_state.send(event).await;
}

pub async fn handle_mouse_view_event(event: MouseEvent, app_state: &mut AppState) {
    match event {
        MouseEvent::OnPress(is_primary) => handle_did_select(is_primary, app_state).await,
        MouseEvent::OnMove(c, r) => handle_did_move(c, r, app_state).await,
    };
}
