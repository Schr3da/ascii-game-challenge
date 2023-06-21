use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_pop_command_system(subscriber: Res<Subscriber>, mut state: ResMut<CommandState>) {
    match subscriber.next_event {
        Some(SendEvents::Commands(CommandInputEvents::Pop)) => {}
        _ => return,
    };

    state.current_inputs.pop();
}
