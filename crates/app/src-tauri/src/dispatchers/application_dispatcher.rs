use core_state::prelude::AppState;
use tauri::Window;

use core_dtos::prelude::*;
use core_state::prelude::*;

pub struct ApplicationDispatcher;

impl ApplicationDispatcher {
    pub fn did_start() -> bool {
        true
    }

    pub async fn did_initialise(state: &mut AppState) -> bool {
        let next = SendEvents::Renderer(RenderEvents::OnWorldWillUpdate);
        state.send(next).await;
        true
    }

    pub fn assets_did_load(event: &GeneralSubscription, window: &Window) -> bool {
        _ = window.emit(&EcsSubscriptionIds::GeneralSubscription.to_string(), event);
        true
    }

    pub fn did_close() -> bool {
        false
    }
}
