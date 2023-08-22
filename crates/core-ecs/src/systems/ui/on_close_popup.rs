use bevy_ecs::prelude::*;

use crate::prelude::*;
use core_dtos::prelude::*;

pub fn on_close_popup_system(mut store: ResMut<UiStore>, subscriber: Res<Subscriber>) {
    match &subscriber.next_event {
        Some(SendEvents::Ui(UiEvents::OnClosePopup)) => {}
        _ => return,
    };

    store.current_popup = None;
}
