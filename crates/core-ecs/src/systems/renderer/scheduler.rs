use bevy_ecs::prelude::*;

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

impl Scheduler for RenderScheduler {
    fn setup(&mut self) {
        self.schedule.add_systems((
            on_update_selected_cell_position_system,
            on_update_selected_cell_asset_system.after(on_update_selected_cell_position_system),
            on_update_cells_system.after(on_update_selected_cell_asset_system),
            on_renderer_did_update_system.after(on_update_cells_system),
        ));
    }

    fn run(&mut self, world: &mut World) {
        self.schedule.run(world);
    }
}
