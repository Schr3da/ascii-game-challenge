use bevy_ecs::prelude::*;

use crate::prelude::*;

pub struct GeneralScheduler {
    pub schedule: Schedule,
}

impl Default for GeneralScheduler {
    fn default() -> Self {
        let schedule = Schedule::default();

        GeneralScheduler { schedule }
    }
}

impl Scheduler for GeneralScheduler {
    fn setup(&mut self) {
        self.schedule.add_systems((
            on_application_will_close_system,
            on_application_did_close_system.after(on_application_will_close_system),
        ));
    }

    fn run(&mut self, world: &mut World) {
        self.schedule.run(world);
    }
}
