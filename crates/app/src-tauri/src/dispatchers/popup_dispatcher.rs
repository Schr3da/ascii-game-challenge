use tauri::Window;

use core_dtos::prelude::*;
use core_state::prelude::*;

pub struct PopupDispatcher;

impl PopupDispatcher {
    pub fn send_popup_data(
        next: &Option<UiView>,
        app_state: &mut AppState,
        window: &Window,
    ) -> bool {
        let previous = app_state.ecs_current_popup.clone();

        match next {
            Some(view) => {
                app_state.ecs_current_popup_state = Some(view.state.clone());
                app_state.ecs_current_popup = next.clone();
            }
            None => {
                app_state.ecs_current_popup_state = None;
                app_state.ecs_current_popup = None;
            }
        };

        if previous == app_state.ecs_current_popup {
            return true;
        }

        _ = window.emit(&EcsSubscriptionIds::PopupSubscription.to_string(), next);

        true
    }
}
