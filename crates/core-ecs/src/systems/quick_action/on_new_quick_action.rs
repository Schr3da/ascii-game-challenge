use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_new_quick_action_system(subscriber: Res<Subscriber>, mut store: ResMut<UiStore>) {
    match subscriber.next_event {
        Some(SendEvents::QuickAction(QuickActionEvents::New)) => {}
        _ => return,
    };

    store.current_popup = Some(UiPopupViewIds::QuickAction);
}
