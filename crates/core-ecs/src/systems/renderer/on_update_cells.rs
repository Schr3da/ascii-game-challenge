use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_update_cells_system(store: Res<UiStore>, mut views_query: Query<&mut UiView>) {
    if store.current_view != UiViewIds::Game {
        return;
    }

    let current_view = store.current_view.clone();

    let mut view = match views_query.iter_mut().find(|v| v.id == current_view) {
        Some(v) => v,
        None => return,
    };

    for child in &mut view.children {
        match child {
            UiViewChild::GameCanvas(_, _) => {
                *child = UiViewChild::GameCanvas(store.width, store.height);
                break;
            }
            _ => {}
        };
    }
}
