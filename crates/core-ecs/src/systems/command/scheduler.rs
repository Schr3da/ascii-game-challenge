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
            on_new_command_system,
            on_cancel_command_system,
            on_push_command_system.after(on_new_command_system),
            on_pop_command_system.after(on_push_command_system),
            on_execute_command_system.after(on_pop_command_system),
            on_command_did_update_system.after(on_execute_command_system),
        ));
    }

    fn run(&mut self, world: &mut World) {
        self.schedule.run(world);
    }
}
