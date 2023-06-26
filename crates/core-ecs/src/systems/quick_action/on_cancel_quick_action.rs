use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_cancel_quick_action_system(subscriber: Res<Subscriber>, mut store: ResMut<UiStore>) {
    match subscriber.next_event {
        Some(SendEvents::QuickAction(QuickActionEvents::Cancel)) => {}
        _ => return,
    };

    store.current_popup = None;
}
