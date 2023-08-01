use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_click_system(mut store: ResMut<UiStore>, subscriber: Res<Subscriber>) {
    let next = match &subscriber.next_event {
        Some(SendEvents::Ui(UiEvents::OnClick(id))) => id,
        _ => return,
    };

    set_next_view(next.clone(), &mut store, &subscriber);
}
