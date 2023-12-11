use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub struct NavigationSystem {}

impl NavigationSystem {
    fn handle_up_navigation(
        cell: &mut SelectedCell,
        camera: &CameraResource,
        assets: &AssetResources,
    ) {
        let previous = cell.frame.y;
        let next = cell.frame.y - cell.frame.height;

        if next >= 0 {
            cell.frame.y = next;
        } else {
            cell.frame.y =
                camera.viewport.height - cell.bottom as i32 - cell.top as i32 - cell.frame.height;
        }

        if assets.terrain.is_visible(cell.frame.x, cell.frame.y) {
            return;
        }

        cell.frame.y = previous;
    }

    fn handle_down_navigation(
        cell: &mut SelectedCell,
        camera: &CameraResource,
        assets: &AssetResources,
    ) {
        let previous = cell.frame.y;
        let next = cell.frame.y + cell.frame.height;

        if next >= camera.viewport.height - cell.bottom as i32 - cell.frame.height {
            cell.frame.y = 0;
        } else {
            cell.frame.y = next;
        }

        if assets.terrain.is_visible(cell.frame.x, cell.frame.y) {
            return;
        }

        cell.frame.y = previous;
    }

    fn handle_left_navigation(
        cell: &mut SelectedCell,
        camera: &CameraResource,
        assets: &AssetResources,
    ) {
        let previous = cell.frame.x;
        let next = cell.frame.x - cell.frame.width;

        if next < 0 {
            cell.frame.x = camera.viewport.width - cell.frame.width;
        } else {
            cell.frame.x = next;
        }

        if assets.terrain.is_visible(cell.frame.x, cell.frame.y) {
            return;
        }

        cell.frame.x = previous;
    }

    fn handle_right_navigation(
        cell: &mut SelectedCell,
        camera: &CameraResource,
        assets: &AssetResources,
    ) {
        let previous = cell.frame.x;
        let next = cell.frame.x + cell.frame.width;

        if next >= camera.viewport.width {
            cell.frame.x = 0;
        } else {
            cell.frame.x = next;
        }

        if assets.terrain.is_visible(cell.frame.x, cell.frame.y) {
            return;
        }

        cell.frame.x = previous;
    }

    fn handle_custom_navigation(
        cell: &mut SelectedCell,
        camera: &CameraResource,
        assets: &AssetResources,
        x: &i32,
        y: &i32,
    ) {
        let previous_x = cell.frame.x;
        let previous_y = cell.frame.y;

        let next_x = *x;
        let next_y = *y - cell.top as i32;

        if next_x >= camera.viewport.width {
            cell.frame.x = camera.viewport.width - cell.frame.width;
        } else {
            cell.frame.x = next_x;
        }

        if next_y < 0 {
            cell.frame.y = cell.top as i32 - cell.frame.height;
        } else if next_y >= camera.viewport.height - cell.bottom as i32 - cell.frame.height {
            cell.frame.y =
                camera.viewport.height - cell.bottom as i32 - cell.top as i32 - cell.frame.height;
        } else {
            cell.frame.y = y - cell.top as i32;
        }

        if assets.terrain.is_visible(cell.frame.x, cell.frame.y) {
            return;
        }

        cell.frame.x = previous_x;
        cell.frame.y = previous_y;
    }

    pub fn on_update_selected_cell_position(
        mut store: ResMut<UiStoreResource>,
        camera: Res<CameraResource>,
        assets: Res<AssetResources>,
        subscriber: Res<SubscriberResource>,
    ) {
        let next = match &subscriber.next_event {
            Some(SendEvents::Renderer(RenderEvents::OnUpdateSelectedCell(e))) => e,
            _ => return,
        };

        let cell = match &mut store.selected_game_tile {
            Some(c) => c,
            None => return,
        };

        match next {
            Navigation::Up => Self::handle_up_navigation(cell, &camera, &assets),
            Navigation::Down => Self::handle_down_navigation(cell, &camera, &assets),
            Navigation::Left => Self::handle_left_navigation(cell, &camera, &assets),
            Navigation::Right => Self::handle_right_navigation(cell, &camera, &assets),
            Navigation::Custom(x, y) => {
                Self::handle_custom_navigation(cell, &camera, &assets, x, y)
            }
        }
    }
}
