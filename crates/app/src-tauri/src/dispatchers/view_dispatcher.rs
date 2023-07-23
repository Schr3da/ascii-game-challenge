use tauri::Window;

use core_dtos::prelude::*;
use core_state::prelude::*;

pub fn dispatch_view_data(
    view: &UiView,
    status: &GameStatus,
    app_state: &mut AppState,
    window: &Window,
) -> bool {
    app_state.ecs_current_game_status = status.clone();
    _ = window.emit(&EcsSubscriptionIds::GameStatusSubscription.to_string(), status);

    app_state.ecs_current_view_state = Some(view.state.clone());

    let should_rerender = match &app_state.ecs_current_view {
        Some(v) => v != view,
        None => true,
    };

    if !should_rerender {
        return true;
    }

    app_state.ecs_current_view = Some(view.clone());

    _ = window.emit(&EcsSubscriptionIds::ViewSubscription.to_string(), view);
    true
}