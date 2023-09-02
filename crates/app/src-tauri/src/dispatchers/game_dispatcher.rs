use tauri::Window;

use core_dtos::prelude::*;
use core_state::prelude::*;

pub struct GameDispatcher;

impl GameDispatcher {
    pub fn send_meta_data(meta: &GameMeta, app_state: &mut AppState, window: &Window) {
        app_state.ecs_current_game_status = meta.status.clone();

        _ = window.emit(&EcsSubscriptionIds::GameMetaSubscription.to_string(), meta);
    }
}
