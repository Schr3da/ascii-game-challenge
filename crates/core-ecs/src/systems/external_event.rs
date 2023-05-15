use bevy_ecs::prelude::*;

use crate::prelude::*;

pub fn external_event_system(subscription: ResMut<Subscriber>) {
    let _ = subscription
        .sender
        .blocking_send(ExternalEvents::Subscriber(
            SubscriptionEvents::OnWorldDidUpdate,
        ));
}
