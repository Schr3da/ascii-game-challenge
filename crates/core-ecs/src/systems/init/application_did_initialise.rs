use bevy_ecs::system::ResMut;

use crate::prelude::*;

pub fn application_did_initialise(subscription: ResMut<Subscriber>) {
    let _ = subscription
        .sender
        .blocking_send(ExternalEvents::Subscriber(SubscriptionEvents::General(
            GeneralSubscription::OnApplicationDidInitialise,
        )));
}
