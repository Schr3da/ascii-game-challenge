use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use core_terrain::prelude::*;

use crate::prelude::*;

pub struct CameraSystems;

impl CameraSystems {
    fn calcualte_position_x(direction: Navigation, camera: &ResMut<CameraResource>) -> Position {
        let step = (camera.viewport.width as f32 * 0.5) as i32;
        let limit = MAP_SIZE as i32;

        let next_x = match direction {
            Navigation::Left => {
                let next = camera.position.x - step;
                if next < 0 {
                    0
                } else {
                    next
                }
            }
            Navigation::Right => {
                if camera.position.x + camera.viewport.width + step >= limit {
                    limit - camera.viewport.width
                } else {
                    camera.position.x + step
                }
            }
            Navigation::Down | Navigation::Up | Navigation::Custom(_, _) => {
                return camera.position.clone()
            }
        };

        Position {
            x: next_x,
            y: camera.position.y,
        }
    }

    fn calcualte_position_y(direction: Navigation, camera: &ResMut<CameraResource>) -> Position {
        let step = (camera.viewport.height as f32 * 0.5) as i32;
        let limit = MAP_SIZE as i32;

        let next_y = match direction {
            Navigation::Up => {
                let next = camera.position.y - step;
                if next < 0 {
                    0
                } else {
                    next
                }
            }
            Navigation::Down => {
                if camera.position.y + camera.viewport.height + step >= limit {
                    limit - camera.viewport.height
                } else {
                    camera.position.y + step
                }
            }
            Navigation::Left | Navigation::Right | Navigation::Custom(_, _) => {
                return camera.position.clone()
            }
        };

        Position {
            x: camera.position.x,
            y: next_y,
        }
    }

    pub fn on_update_camera(
        mut camera: ResMut<CameraResource>,
        subscriber: Res<SubscriberResource>,
    ) {
        let next = match &subscriber.next_event {
            Some(SendEvents::Renderer(RenderEvents::OnUpdateCamera(n))) => n.clone(),
            _ => {
                return;
            }
        };

        camera.position = match next {
            Navigation::Up | Navigation::Down => Self::calcualte_position_y(next, &camera),
            Navigation::Left | Navigation::Right => Self::calcualte_position_x(next, &camera),
            Navigation::Custom(_, _) => return,
        };
    }
}
