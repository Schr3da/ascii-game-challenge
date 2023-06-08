use bevy_ecs::prelude::*;

use crate::prelude::*;
use core_dtos::prelude::*;

pub fn on_close_view_system(mut store: ResMut<UiStore>, subscriber: Res<Subscriber>) {
    match &subscriber.next_event {
        Some(SendEvents::Ui(UiEvents::OnCloseView)) => {}
        _ => return,
    };

    match store.previous_view.pop() {
        Some(v) => store.current_view = v,
        None => store.current_view = UiViewIds::Main,
    }
}
