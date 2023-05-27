use bevy_ecs::system::ResMut;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_application_will_resize_system(
    mut store: ResMut<UiStore>,
    subscription: ResMut<Subscriber>,
) {
    let (width, height) = match subscription.next_event {
        Some(SendEvents::General(GeneralEvents::OnApplicationResize(w, h))) => (w, h),
        _ => return,
    };

    store.width = width;
    store.height = height;

    _ = subscription
        .sender
        .blocking_send(EcsEvents::Send(SendEvents::Renderer(
            RenderEvents::OnWorldWillUpdate,
        )));
}
