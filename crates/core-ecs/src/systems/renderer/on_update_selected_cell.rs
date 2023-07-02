use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_update_selected_cell_system(
    mut store: ResMut<UiStore>,
    camera: Res<Camera2d>,
    subscriber: Res<Subscriber>,
) {
    let next = match &subscriber.next_event {
        Some(SendEvents::Renderer(RenderEvents::OnUpdateSelectedCell(e))) => e,
        _ => return,
    };

    let mut cell = match &mut store.selected_game_tile {
        Some(c) => c,
        None => return,
    };

    if next == &SelectedCellNavigation::Up {
        let next_y = cell.frame.y - cell.frame.height;
        if next_y < 0 {
            cell.frame.y =
                camera.viewport.height - cell.bottom as i32 - cell.top as i32 - cell.frame.height;
        } else {
            cell.frame.y = next_y;
        }

        return;
    }

    if next == &SelectedCellNavigation::Left {
        let next_x = cell.frame.x - cell.frame.width;
        if next_x < 0 {
            cell.frame.x = camera.viewport.width - cell.frame.width;
        } else {
            cell.frame.x = next_x;
        }

        return;
    }

    if next == &SelectedCellNavigation::Down {
        let next_y = cell.frame.y + cell.frame.height;
        if next_y >= camera.viewport.height - cell.bottom as i32 - cell.frame.height {
            cell.frame.y = 0;
        } else {
            cell.frame.y = next_y;
        }

        return;
    }

    if next == &SelectedCellNavigation::Right {
        let next_x = cell.frame.x + cell.frame.width;
        if next_x >= camera.viewport.width {
            cell.frame.x = 0;
        } else {
            cell.frame.x = next_x;
        }

        return;
    }

    if let SelectedCellNavigation::Custom(column, row) = next {
        let next_x = *column;

        if next_x >= camera.viewport.width {
            cell.frame.x = camera.viewport.width - cell.frame.width;
        } else {
            cell.frame.x = next_x;
        }

        let next_y = *row - cell.top as i32;

        if next_y < 0 {
            cell.frame.y = cell.top as i32 - cell.frame.height;
        } else if next_y >= camera.viewport.height - cell.bottom as i32 - cell.frame.height {
            cell.frame.y =
                camera.viewport.height - cell.bottom as i32 - cell.top as i32 - cell.frame.height;
        } else {
            cell.frame.y = *row - cell.top as i32;
        }

        return;
    }
}
