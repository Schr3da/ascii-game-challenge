use bevy_ecs::prelude::*;

use crate::prelude::*;

pub struct InputScheduler {
    pub schedule: Schedule,
}

impl Default for InputScheduler {
    fn default() -> Self {
        let schedule = Schedule::default();

        InputScheduler { schedule }
    }
}

impl Scheduler for InputScheduler {
    fn setup(&mut self) {
        self.schedule.add_systems((
            on_new_input_system,
            on_cancel_input_system,
            on_push_input_system.after(on_new_input_system),
            on_pop_input_system.after(on_push_input_system),
            on_execute_input_system.after(on_pop_input_system),
            on_input_did_update_system.after(on_execute_input_system),
        ));
    }

    fn run(&mut self, world: &mut World) {
        self.schedule.run(world);
    }
}
