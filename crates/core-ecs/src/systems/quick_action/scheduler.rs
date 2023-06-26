use bevy_ecs::prelude::*;

use crate::prelude::*;

pub struct QuickActionScheduler {
    pub schedule: Schedule,
}

impl Default for QuickActionScheduler {
    fn default() -> Self {
        let schedule = Schedule::default();

        QuickActionScheduler { schedule }
    }
}

impl Scheduler for QuickActionScheduler {
    fn setup(&mut self) {
        self.schedule.add_systems((
            on_new_quick_action_system,
            on_cancel_quick_action_system.after(on_new_quick_action_system),
            on_quick_action_did_update_system.after(on_cancel_quick_action_system),
        ));
    }

    fn run(&mut self, world: &mut World) {
        self.schedule.run(world);
    }
}
