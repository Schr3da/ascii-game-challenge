use bevy_ecs::schedule::*;

pub struct InitScheduler {
    pub schedule: Schedule,
}

impl Default for InitScheduler {
    fn default() -> Self {
        let schedule = Schedule::default();

        InitScheduler { schedule }
    }
}
