use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_new_command_system(
    subscriber: Res<Subscriber>,
    store: ResMut<UiStore>,
    views: Query<&mut UiView>,
) {
    match subscriber.next_event {
        Some(SendEvents::Commands(CommandInputEvents::New)) => {}
        _ => return,
    };

    set_next_popup(UiPopupViewIds::Command, store, views);
}
