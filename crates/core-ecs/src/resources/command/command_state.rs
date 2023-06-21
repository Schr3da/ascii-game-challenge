use bevy_ecs::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct CommandState {
    pub current_inputs: Vec<String>,
}
