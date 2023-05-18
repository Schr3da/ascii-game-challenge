use bevy_ecs::prelude::*;

use crate::prelude::*;

pub struct UiScheduler {
    pub schedule: Schedule,
}

impl Default for UiScheduler {
    fn default() -> Self {
        let schedule = Schedule::default();

        UiScheduler{ schedule }
    }
}

impl Scheduler for UiScheduler{
    fn register(&mut self) {
        self.schedule.add_systems((
            cell_renderer_system,
            external_event_system.after(cell_renderer_system),
        ));
    }

    fn unregister(&mut self) {
        todo!()
    }

    fn run(&mut self, world: &mut World) {
        self.schedule.run(world);
    }
}