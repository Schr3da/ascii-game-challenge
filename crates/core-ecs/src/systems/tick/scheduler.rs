use bevy_ecs::prelude::*;

use crate::prelude::*;

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

impl Scheduler for TickScheduler {
    fn setup(&mut self) {
        self.schedule.add_systems((
            on_update_view_labels_system,
            on_tick_did_complete_system.after(on_update_view_labels_system),
        ));
    }

    fn run(&mut self, world: &mut World) {
        self.schedule.run(world);
    }
}
