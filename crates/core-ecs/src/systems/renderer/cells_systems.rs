use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub struct CellsSystems;

impl CellsSystems {
    fn canvas_frame(camera: &Res<CameraResource>, top: u16, bottom: u16) -> Rect {
        Rect {
            x: 0,
            y: top as i32,
            width: camera.viewport.width as i32,
            height: camera.viewport.height - bottom as i32,
        }
    }

    fn visible_canvas_cells(
        frame: &Rect,
        camera: &Res<CameraResource>,
        assets: &Res<AssetResources>,
    ) -> Vec<(Cell, Position)> {
        let mut next = Vec::new();

        for y in frame.y..frame.height {
            for x in 0..frame.width {
                let ascii = assets
                    .terrain
                    .get_ascii(camera.position.x + x, camera.position.y + y);
                let position = Position { x, y };

                if let Some(c) = assets.cell_cache.get(&ascii) {
                    next.push((c.clone(), position));
                }
            }
        }

        next
    }

    pub fn on_update_cells(
        assets: Res<AssetResources>,
        camera: Res<CameraResource>,
        mut store: ResMut<UiStoreResource>,
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
                UiViewChild::GameCanvas(_) => {
                    let frame = Self::canvas_frame(&camera, top, bottom);
                    let visible_cells = Self::visible_canvas_cells(&frame, &camera, &assets);

                    *child = UiViewChild::GameCanvas(visible_cells);

                    break;
                }
                _ => {}
            };
        }
    }

    pub fn on_update_selected_cell_asset(
        mut store: ResMut<UiStoreResource>,
        camera: Res<CameraResource>,
        assets: Res<AssetResources>,
    ) {
        let tile = match &mut store.selected_game_tile {
            Some(c) => c,
            None => return,
        };

        let next = assets.terrain.get_ascii(
            camera.position.x + tile.frame.x,
            camera.position.y + tile.frame.y + tile.top as i32,
        );

        let mut cell = assets
            .cell_cache
            .get(&next)
            .unwrap_or(&Cell::default())
            .clone();

        cell.background = CellColors::Rgb(0, 0, 0);
        cell.foreground = CellColors::Rgb(255, 255, 255);

        tile.cell = cell;
    }

    pub fn on_update_selected_cell_position(
        mut store: ResMut<UiStoreResource>,
        camera: Res<CameraResource>,
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

        if next == &Navigation::Up {
            let next_y = cell.frame.y - cell.frame.height;
            if next_y < 0 {
                cell.frame.y = camera.viewport.height
                    - cell.bottom as i32
                    - cell.top as i32
                    - cell.frame.height;
            } else {
                cell.frame.y = next_y;
            }

            return;
        }

        if next == &Navigation::Left {
            let next_x = cell.frame.x - cell.frame.width;
            if next_x < 0 {
                cell.frame.x = camera.viewport.width - cell.frame.width;
            } else {
                cell.frame.x = next_x;
            }

            return;
        }

        if next == &Navigation::Down {
            let next_y = cell.frame.y + cell.frame.height;
            if next_y >= camera.viewport.height - cell.bottom as i32 - cell.frame.height {
                cell.frame.y = 0;
            } else {
                cell.frame.y = next_y;
            }

            return;
        }

        if next == &Navigation::Right {
            let next_x = cell.frame.x + cell.frame.width;
            if next_x >= camera.viewport.width {
                cell.frame.x = 0;
            } else {
                cell.frame.x = next_x;
            }

            return;
        }

        if let Navigation::Custom(column, row) = next {
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
                cell.frame.y = camera.viewport.height
                    - cell.bottom as i32
                    - cell.top as i32
                    - cell.frame.height;
            } else {
                cell.frame.y = *row - cell.top as i32;
            }

            return;
        }
    }
}
