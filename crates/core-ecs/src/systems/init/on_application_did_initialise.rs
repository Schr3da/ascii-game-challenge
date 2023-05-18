use bevy_ecs::system::ResMut;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_application_did_initialise_system(subscription: ResMut<Subscriber>) {
    _ = subscription
        .sender
        .blocking_send(EcsEvents::Subscriber(SubscriptionEvents::General(
            GeneralSubscription::OnApplicationDidInitialise,
        )));
}
