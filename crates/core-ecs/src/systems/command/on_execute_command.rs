use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_execute_command_system(
    subscriber: Res<Subscriber>,
    mut store: ResMut<UiStore>,
    mut state: ResMut<CommandState>,
) {
    _ = match &subscriber.next_event {
        Some(SendEvents::Commands(CommandInputEvents::Execute(n))) => n,
        _ => return,
    };

    store.current_popup = None;
    state.current_inputs = Vec::new();
}
