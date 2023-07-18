use bevy_ecs::prelude::*;
use core_dtos::prelude::{Cell, CellColors};

use crate::prelude::*;

pub fn on_update_selected_cell_asset_system(
    mut store: ResMut<UiStore>,
    camera: Res<Camera2d>,
    assets: Res<AssetResources>,
) {
    let mut tile = match &mut store.selected_game_tile {
        Some(c) => c,
        None => return,
    };

    let next = assets
        .terrain
        .get_ascii(camera.position.x + tile.frame.x, camera.position.y + tile.frame.y + tile.top as i32);

    let mut cell = assets
        .cell_cache
        .get(&next)
        .unwrap_or(&Cell::default())
        .clone();

    cell.background = CellColors::Rgb(0, 0, 0);
    cell.foreground = CellColors::Rgb(255, 255, 255);

    tile.cell = cell;
}
