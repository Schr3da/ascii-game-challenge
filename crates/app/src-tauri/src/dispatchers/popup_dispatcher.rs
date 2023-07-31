use tauri::Window;

use core_dtos::prelude::*;
use core_state::prelude::*;

pub fn dispatch_popup_data(view: &UiView, app_state: &mut AppState, window: &Window) -> bool {
    app_state.ecs_current_popup_state = Some(view.state.clone());

    let should_rerender = match &app_state.ecs_current_popup {
        Some(v) => v != view,
        None => true,
    };

    if should_rerender {
        return true;
    }

    app_state.ecs_current_popup = Some(view.clone());

    _ = window.emit(&EcsSubscriptionIds::PopupSubscription.to_string(), view);

    true
}
