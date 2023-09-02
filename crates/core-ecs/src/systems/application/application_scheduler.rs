use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

use crate::prelude::*;

pub struct ApplicationScheduler {
    pub schedule: Schedule,
}

impl Default for ApplicationScheduler {
    fn default() -> Self {
        let schedule = Schedule::default();

        ApplicationScheduler { schedule }
    }
}

impl ToScheduler for ApplicationScheduler {
    fn setup(&mut self) {
        self.schedule.add_systems((
            ApplicationSystems::on_application_will_resize,
            ApplicationSystems::on_application_will_close,
        ));
    }

    fn run(&mut self, world: &mut World) {
        self.schedule.run(world);
    }
}
