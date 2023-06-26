use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_quick_action_did_update_system(subscription: Res<Subscriber>) {
    _ = subscription
        .sender
        .blocking_send(EcsEvents::Send(SendEvents::Renderer(
            RenderEvents::OnWorldWillUpdate,
        )));
}
