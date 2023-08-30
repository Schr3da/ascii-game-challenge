use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

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

impl ToScheduler for InitScheduler {
    fn setup(&mut self) {
        self.schedule.add_systems((
            InitSystems::on_application_will_initialise,
            InitSystems::on_application_will_load_assets
                .after(InitSystems::on_application_will_initialise),
            InitSystems::on_application_did_initialise
                .after(InitSystems::on_application_will_load_assets),
        ));
    }

    fn run(&mut self, world: &mut World) {
        self.schedule.run(world);
    }
}
