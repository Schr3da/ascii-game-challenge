use bevy_ecs::prelude::*;

pub trait Scheduler {
    fn setup(&mut self);

    fn run(&mut self, world: &mut World);
}
