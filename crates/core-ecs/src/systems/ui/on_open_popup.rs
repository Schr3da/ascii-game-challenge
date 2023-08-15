use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_open_popup_system(mut store: ResMut<UiStore>, subscriber: Res<Subscriber>) {
    let next = match &subscriber.next_event {
        Some(SendEvents::Ui(UiEvents::OnOpenPopup(e))) => e,
        _ => return,
    };

    store.current_popup = Some(next.clone());
}
