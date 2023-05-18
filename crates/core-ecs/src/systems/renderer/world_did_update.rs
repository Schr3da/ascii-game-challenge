use bevy_ecs::prelude::*;

use crate::prelude::*;

pub fn world_did_update_system(subscription: ResMut<Subscriber>) {
    let _ = subscription
        .sender
        .blocking_send(ExternalEvents::Subscriber(SubscriptionEvents::Renderer(
            RenderSubscription::OnWorldDidUpdate,
        )));
}
