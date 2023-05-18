use bevy_ecs::system::ResMut;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn application_did_close(subscription: ResMut<Subscriber>) {
    _ = subscription
        .sender
        .blocking_send(EcsEvents::Subscriber(SubscriptionEvents::General(
            GeneralSubscription::OnApplicationDidClose,
        )));
}
