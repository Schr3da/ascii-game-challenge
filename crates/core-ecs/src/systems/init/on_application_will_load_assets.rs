use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_application_will_load_assets(subscription: Res<Subscriber>, assets: Res<AssetResources>) {
    match subscription.next_event {
        Some(SendEvents::General(GeneralEvents::OnApplicationWillInitialise(_, _))) => {}
        _ => return,
    };

    let cells = assets.cell_cache.clone();

    _ = subscription
        .sender
        .blocking_send(EcsEvents::Subscriber(SubscriptionEvents::General(
            GeneralSubscription::OnApplicationDidLoadAssets(cells),
        )));
}
