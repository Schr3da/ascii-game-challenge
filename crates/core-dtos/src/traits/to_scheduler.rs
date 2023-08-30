use bevy_ecs::prelude::*;

pub trait ToScheduler {
    fn setup(&mut self);

    fn run(&mut self, world: &mut World);
}
