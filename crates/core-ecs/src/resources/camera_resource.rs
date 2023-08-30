use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

#[derive(Debug, Resource)]
pub struct CameraResource {
    pub viewport: Rect,
    pub position: Position,
}

impl Default for CameraResource {
    fn default() -> Self {
        CameraResource {
            viewport: Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            position: Position { x: 0, y: 0 },
        }
    }
}
