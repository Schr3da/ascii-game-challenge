use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_click_system(mut store: ResMut<UiStore>, views_query: Query<&UiView>) {
    let view = match views_query.iter().find(|v| v.id == store.current_view) {
        Some(v) => v,
        None => return,
    };

    match view.state.selected_id {
        ViewComponentIds::Main(MainMenu::NewGame) => {
            store.current_view = UiViewIds::Game;
        }
        ViewComponentIds::Main(MainMenu::Options) => {
            store.current_view = UiViewIds::Options;
        }
        _ => return,
    };
}
