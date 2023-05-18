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
    fn register(&mut self) {
        self.schedule.add_systems((
            application_will_close,
            application_did_close.after(application_will_close),
        ));
    }

    fn unregister(&mut self) {
        todo!()
    }

    fn run(&mut self, world: &mut World) {
        self.schedule.run(world);
    }
}
