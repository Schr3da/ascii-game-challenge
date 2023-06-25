use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_update_selected_cell_system(mut store: ResMut<UiStore>, subscriber: Res<Subscriber>) {
    let next = match &subscriber.next_event {
        Some(SendEvents::Renderer(RenderEvents::OnUpdateSelectedCell(e))) => e,
        _ => return,
    };

    let mut cell = match &mut store.selected_game_tile {
        Some(c) => c,
        None => return,
    };

    if next == &SelectedCellNavigation::Up {
        let next_y = cell.frame.y - 1;
        cell.frame.y = next_y;
    }

    if next == &SelectedCellNavigation::Down {
        let next_y = cell.frame.y + 1;
        cell.frame.y = next_y;
    }

    if next == &SelectedCellNavigation::Left {
        let next_x = cell.frame.x - 1;
        cell.frame.x = next_x;
    }

    if next == &SelectedCellNavigation::Right {
        let next_x = cell.frame.x + 1;
        cell.frame.x = next_x;
    }
}
