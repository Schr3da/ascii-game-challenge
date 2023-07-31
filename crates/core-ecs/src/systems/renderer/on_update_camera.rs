use bevy_ecs::prelude::*;

use core_dtos::prelude::*;
use core_terrain::prelude::MAP_SIZE;

use crate::prelude::*;

fn calcualte_position_x(direction: CameraNavigation, camera: &ResMut<Camera2d>) -> Position {
    let step = (camera.viewport.width as f32 * 0.5) as i32;
    let limit = MAP_SIZE as i32;

    let next_x = match direction {
        CameraNavigation::Left => {
            let next = camera.position.x - step;
            if next < 0 {
                0
            } else {
                next
            }
        }
        CameraNavigation::Right => {
            if camera.position.x + camera.viewport.width + step >= limit {
                limit - camera.viewport.width
            } else {
                camera.position.x + step
            }
        }
        CameraNavigation::Down | CameraNavigation::Up => return camera.position.clone(),
    };

    Position {
        x: next_x,
        y: camera.position.y,
    }
}

fn calcualte_position_y(direction: CameraNavigation, camera: &ResMut<Camera2d>) -> Position {
    let step = (camera.viewport.height as f32 * 0.5) as i32;
    let limit = MAP_SIZE as i32;

    let next_y = match direction {
        CameraNavigation::Up => {
            let next = camera.position.y - step;
            if next < 0 {
                0
            } else {
                next
            }
        }
        CameraNavigation::Down => {
            if camera.position.y + camera.viewport.height + step >= limit {
                limit - camera.viewport.height
            } else {
                camera.position.y + step
            }
        }
        CameraNavigation::Left | CameraNavigation::Right => return camera.position.clone(),
    };

    Position {
        x: camera.position.x,
        y: next_y,
    }
}

pub fn on_update_camera_system(mut camera: ResMut<Camera2d>, subscriber: Res<Subscriber>) {
    let next = match &subscriber.next_event {
        Some(SendEvents::Renderer(RenderEvents::OnUpdateCamera(n))) => n.clone(),
        _ => {
            return;
        }
    };

    camera.position = match next {
        CameraNavigation::Up | CameraNavigation::Down => calcualte_position_y(next, &camera),
        CameraNavigation::Left | CameraNavigation::Right => calcualte_position_x(next, &camera),
    };
}
