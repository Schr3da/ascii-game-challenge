use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

#[derive(Resource)]
pub struct Camera2d {
    pub viewport: Rect,
    pub position: Position,
}

impl Default for Camera2d {
    fn default() -> Self {
        Camera2d {
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