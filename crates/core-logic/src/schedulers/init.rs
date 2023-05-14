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
    fn register(&mut self) {
        self.schedule.add_systems(cell_renderer_system());
    }

    fn unregister(&mut self) {
        todo!()
    }

    fn run(&mut self, world: &mut World) {
        self.schedule.run(world)
    }
}
