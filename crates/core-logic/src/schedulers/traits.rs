use bevy_ecs::prelude::*;

pub trait Scheduler {
    fn register(&mut self);

    fn unregister(&mut self);

    fn run(&mut self, world: &mut World);
}
