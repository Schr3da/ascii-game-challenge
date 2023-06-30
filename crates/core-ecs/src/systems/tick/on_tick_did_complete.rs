use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_tick_did_complete_system(subscription: Res<Subscriber>) {
    _ = subscription
        .sender
        .blocking_send(EcsEvents::Send(SendEvents::Renderer(
            RenderEvents::OnWorldWillUpdate,
        )));
}
