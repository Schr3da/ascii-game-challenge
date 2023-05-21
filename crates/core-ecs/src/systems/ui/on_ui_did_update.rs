use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_ui_did_update_system(
    subscription: Res<Subscriber>,
    store: Res<UiStore>,
    views_query: Query<&UiView>,
) {
    let view = views_query
        .iter()
        .find(|v| v.id == store.current_view)
        .cloned();

    let current = subscription.next_event.clone();

    _ = subscription
        .sender
        .blocking_send(EcsEvents::Subscriber(SubscriptionEvents::Renderer(
            RenderSubscription::OnWorldDidUpdate(view, current),
        )));
}
