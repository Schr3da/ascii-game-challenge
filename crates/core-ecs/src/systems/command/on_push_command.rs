use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_push_command_system(subscriber: Res<Subscriber>, mut state: ResMut<CommandState>) {
    let next = match &subscriber.next_event {
        Some(SendEvents::Commands(CommandInputEvents::Push(n))) => n,
        _ => return,
    };

    state.current_inputs.push(next.clone());
}
