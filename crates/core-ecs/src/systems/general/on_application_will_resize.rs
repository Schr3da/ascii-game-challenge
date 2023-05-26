use bevy_ecs::system::ResMut;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_application_will_resize_system(subscription: ResMut<Subscriber>) {
    let (_width, _height) = match subscription.next_event {
        Some(SendEvents::General(GeneralEvents::OnApplicationResize(w, h))) => (w, h),
        _ => return,
    };

    _ = subscription
        .sender
        .blocking_send(EcsEvents::Send(SendEvents::Renderer(
            RenderEvents::OnWorldWillUpdate,
        )));
}
