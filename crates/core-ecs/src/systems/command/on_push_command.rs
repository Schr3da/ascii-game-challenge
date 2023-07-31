use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_push_command_system(
    subscriber: Res<Subscriber>,
    mut command_state: ResMut<CommandState>,
) {
    let next = match &subscriber.next_event {
        Some(SendEvents::Commands(CommandInputEvents::Push(n))) => n,
        _ => return,
    };

    command_state.current_inputs.push(next.clone());
}
