use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_renderer_did_update_system(subscription: ResMut<Subscriber>) {
    _ = subscription
        .sender
        .blocking_send(EcsEvents::Subscriber(SubscriptionEvents::Renderer(
            RenderSubscription::OnWorldDidUpdate,
        )));
}
