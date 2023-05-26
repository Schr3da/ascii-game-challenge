use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_application_will_close_system(subscription: Res<Subscriber>) {
    match &subscription.next_event {
        Some(SendEvents::General(GeneralEvents::OnApplicationWillClose)) => {}
        _ => return,
    };

    //add cleanup / save code here

    _ = subscription
        .sender
        .blocking_send(EcsEvents::Subscriber(SubscriptionEvents::General(
            GeneralSubscription::OnApplicationDidClose,
        )));
}
