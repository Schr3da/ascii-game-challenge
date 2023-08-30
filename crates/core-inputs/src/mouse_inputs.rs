use core_dtos::prelude::*;
use core_state::prelude::*;

pub struct MouseInputs;

impl MouseInputs {
    async fn handle_move(column: i32, row: i32, app_state: &mut AppState) {
        app_state
            .send(SendEvents::Renderer(RenderEvents::OnUpdateSelectedCell(
                Navigation::Custom(column, row),
            )))
            .await;
    }

    async fn handle_click(is_primary_button: bool, app_state: &mut AppState) {
        if is_primary_button == false {
            return;
        }

        let event = SendEvents::Ui(UiEvents::OnOpenPopup(UiPopupViewIds::Actions));
        app_state.send(event).await;
    }

    pub async fn handle_event(event: MouseEvent, app_state: &mut AppState) -> bool {
        if app_state.is_popup_visible() {
            return false;
        }

        if !app_state.has_game_started() {
            return false;
        }

        match event {
            MouseEvent::OnPress(is_primary) => Self::handle_click(is_primary, app_state).await,
            MouseEvent::OnMove(c, r) => Self::handle_move(c, r, app_state).await,
        };

        true
    }
}
