use bevy_ecs::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct CommandResource {
    pub current_inputs: Vec<String>,
}
