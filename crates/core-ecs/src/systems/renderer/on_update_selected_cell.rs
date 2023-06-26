use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

const CELL_OFFSET: i32 = 1;

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
        let next_y = cell.frame.y - CELL_OFFSET;
        if next_y < 0 {
            cell.frame.y =
                camera.viewport.height - cell.bottom as i32 - cell.top as i32 - CELL_OFFSET;
        } else {
            cell.frame.y = next_y;
        }

        return;
    }

    if next == &SelectedCellNavigation::Left {
        let next_x = cell.frame.x - CELL_OFFSET;
        if next_x < 0 {
            cell.frame.x = camera.viewport.width - CELL_OFFSET;
        } else {
            cell.frame.x = next_x;
        }

        return;
    }

    if next == &SelectedCellNavigation::Down {
        let next_y = cell.frame.y + 1;
        if next_y >= camera.viewport.height - cell.bottom as i32 - CELL_OFFSET {
            cell.frame.y = 0;
        } else {
            cell.frame.y = next_y;
        }

        return;
    }

    if next == &SelectedCellNavigation::Right {
        let next_x = cell.frame.x + 1;
        if next_x >= camera.viewport.width {
            cell.frame.x = 0;
        } else {
            cell.frame.x = next_x;
        }

        return;
    }
}
