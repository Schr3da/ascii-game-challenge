use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

fn get_canvas_frame(store: Res<UiStore>, top: u16, bottom: u16) -> Rect {
    Rect {
        x: 0,
        y: top as i32,
        width: store.width as i32,
        height: (store.height - bottom) as i32,
    }
}

fn value_to_ascii(value: i32) -> Ascii {
    match value {
        0..=2 => Ascii::Plus,
        _ => Ascii::Space,
    }
}

fn get_canvas_cells(frame: &Rect, assets: &Res<AssetResources>) -> Vec<(Cell, Position)> {
    let mut next = Vec::new();

    for y in frame.y..frame.height {
        for x in 0..frame.width {
            let value = assets.terrain.get_value(x, y);
            let ascii = value_to_ascii(value);
            let position = Position { x, y };

            if let Some(c) = assets.cell_cache.get(&ascii) {
                next.push((c.clone(), position));
            }
        }
    }

    next
}

pub fn on_update_cells_system(
    store: Res<UiStore>,
    assets: Res<AssetResources>,
    mut views_query: Query<&mut UiView>,
) {
    if store.current_view != UiViewIds::Game {
        return;
    }

    let current_view = store.current_view.clone();

    let mut view = match views_query.iter_mut().find(|v| v.id == current_view) {
        Some(v) => v,
        None => return,
    };

    let top = match *view
        .layout
        .constraints
        .first()
        .unwrap_or(&LayoutConstraints::default())
    {
        LayoutConstraints::Value(v) => v,
        _ => 0,
    };

    let bottom = match *view
        .layout
        .constraints
        .last()
        .unwrap_or(&LayoutConstraints::default())
    {
        LayoutConstraints::Value(v) => v,
        _ => 0,
    };

    for child in &mut view.children {
        match child {
            UiViewChild::GameCanvas(_) => {
                let frame = get_canvas_frame(store, top, bottom);
                let cells = get_canvas_cells(&frame, &assets);
                *child = UiViewChild::GameCanvas(cells);
                break;
            }
            _ => {}
        };
    }
}
