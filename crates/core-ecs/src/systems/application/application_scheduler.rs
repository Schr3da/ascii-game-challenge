use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

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

impl ToScheduler for GeneralScheduler {
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
