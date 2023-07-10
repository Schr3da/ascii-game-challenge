use bevy_ecs::prelude::*;

use crate::prelude::*;

pub struct InitScheduler {
    pub schedule: Schedule,
}

impl Default for InitScheduler {
    fn default() -> Self {
        let schedule = Schedule::default();

        InitScheduler { schedule }
    }
}

impl Scheduler for InitScheduler {
    fn setup(&mut self) {
        self.schedule.add_systems((
            on_application_will_initialise_system,
            on_application_will_load_assets.after(on_application_will_initialise_system),
            on_application_did_initialise_system.after(on_application_will_load_assets),
        ));
    }

    fn run(&mut self, world: &mut World) {
        self.schedule.run(world);
    }
}
