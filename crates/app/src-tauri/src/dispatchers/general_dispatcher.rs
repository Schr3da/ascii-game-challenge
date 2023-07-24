use core_state::prelude::AppState;
use tauri::Window;

use core_dtos::prelude::*;
use core_state::prelude::*;

pub fn dispatch_application_did_start() -> bool {
    true
}

pub async fn dispatch_appliaction_did_initialise(state: &mut AppState) -> bool {
    let next = SendEvents::Renderer(RenderEvents::OnWorldWillUpdate);
    state.send(next).await;
    true
}

pub fn dispatch_assets_did_load(event: &GeneralSubscription, window: &Window) -> bool {
    _ = window.emit(&EcsSubscriptionIds::GeneralSubscription.to_string(), event);
    true
}

pub fn dispatch_application_did_close() -> bool {
    false
}
