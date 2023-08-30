use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

use super::prelude::*;

pub struct TickScheduler {
    pub schedule: Schedule,
}

impl Default for TickScheduler {
    fn default() -> Self {
        let schedule = Schedule::default();

        TickScheduler { schedule }
    }
}

impl ToScheduler for TickScheduler {
    fn setup(&mut self) {
        self.schedule.add_systems((
            TickSystems::on_update_view_labels,
            TickSystems::on_tick_did_complete.after(TickSystems::on_update_view_labels),
        ));
    }

    fn run(&mut self, world: &mut World) {
        self.schedule.run(world);
    }
}
