use bevy_ecs::system::ResMut;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_application_will_resize_system(
    mut camera: ResMut<Camera2d>,
    subscription: ResMut<Subscriber>,
) {
    let (width, height) = match subscription.next_event {
        Some(SendEvents::General(GeneralEvents::OnApplicationResize(w, h))) => (w, h),
        _ => return,
    };

    camera.viewport = Rect {
        width: width as i32,
        height: height as i32,
        ..camera.viewport
    };

    _ = subscription
        .sender
        .blocking_send(EcsEvents::Send(SendEvents::Renderer(
            RenderEvents::OnWorldWillUpdate,
        )));
}
