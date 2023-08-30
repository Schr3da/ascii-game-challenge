use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

use crate::prelude::*;

pub struct RenderScheduler {
    pub schedule: Schedule,
}

impl Default for RenderScheduler {
    fn default() -> Self {
        let schedule = Schedule::default();

        RenderScheduler { schedule }
    }
}

impl ToScheduler for RenderScheduler {
    fn setup(&mut self) {
        self.schedule.add_systems((
            CameraSystems::on_update_camera,
            CellsSystems::on_update_selected_cell_position.after(CameraSystems::on_update_camera),
            CellsSystems::on_update_selected_cell_asset
                .after(CellsSystems::on_update_selected_cell_position),
            CellsSystems::on_update_cells.after(CellsSystems::on_update_selected_cell_asset),
            RendererSystems::on_renderer_did_update.after(CellsSystems::on_update_cells),
        ));
    }

    fn run(&mut self, world: &mut World) {
        self.schedule.run(world);
    }
}
