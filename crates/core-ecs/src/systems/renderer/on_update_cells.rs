use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

fn get_canvas_frame(camera: &Res<Camera2d>, top: u16, bottom: u16) -> Rect {
    Rect {
        x: 0,
        y: top as i32,
        width: camera.viewport.width as i32,
        height: camera.viewport.height - bottom as i32,
    }
}

fn get_visible_canvas_cells(frame: &Rect, assets: &Res<AssetResources>) -> Vec<(Cell, Position)> {
    let mut next = Vec::new();

    for y in frame.y..frame.height {
        for x in 0..frame.width {
            let ascii = assets.terrain.get_ascii(x, y);
            let position = Position { x, y };

            if let Some(c) = assets.cell_cache.get(&ascii) {
                next.push((c.clone(), position));
            }
        }
    }

    next
}

pub fn on_update_cells_system(
    assets: Res<AssetResources>,
    camera: Res<Camera2d>,
    mut store: ResMut<UiStore>,
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

    match &mut store.selected_game_tile {
        Some(s) => {
            s.top = top;
            s.bottom = bottom;
        }
        None => {}
    };

    for child in &mut view.children {
        match child {
            UiViewChild::GameCanvas(_, _) => {
                let frame = get_canvas_frame(&camera, top, bottom);
                let visible_cells = get_visible_canvas_cells(&frame, &assets);
                let selected_cell = store.selected_game_tile.clone();

                *child = UiViewChild::GameCanvas(visible_cells, selected_cell);

                break;
            }
            _ => {}
        };
    }
}
