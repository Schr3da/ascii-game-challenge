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

    let _first = view
        .layout
        .constraints
        .first()
        .unwrap_or(&LayoutConstraints::default());

    let _last = view
        .layout
        .constraints
        .last()
        .unwrap_or(&LayoutConstraints::default());

    for child in &mut view.children {
        match child {
            UiViewChild::GameCanvas(_) => {
                *child = UiViewChild::GameCanvas(Rect {
                    x: 0,
                    y: 1,
                    width: store.width as i32,
                    height: (store.height - 1) as i32,
                });
                break;
            }
            _ => {}
        };
    }
}
