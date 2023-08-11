use tauri::Window;

use core_dtos::prelude::*;
use core_state::prelude::*;

pub fn dispatch_view_data(
    view: &Option<UiView>,
    app_state: &mut AppState,
    window: &Window,
) -> bool {
    let next = match view {
        Some(v) => v,
        None => return true,
    };

    let should_rerender = match &app_state.ecs_current_view {
        Some(v) => v != next,
        None => true,
    };

    if !should_rerender {
        return true;
    }

    app_state.ecs_current_view_state = Some(next.state.clone());
    app_state.ecs_current_view = view.clone();

    _ = window.emit(&EcsSubscriptionIds::ViewSubscription.to_string(), view);
    true
}
