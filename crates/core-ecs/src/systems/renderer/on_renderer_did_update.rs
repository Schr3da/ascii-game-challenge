use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_renderer_did_update_system(
    subscription: Res<Subscriber>,
    store: Res<UiStore>,
    views_query: Query<&UiView>,
) {
    let current_view = store.current_view.clone();

    let view = views_query.iter().find(|v| v.id == current_view)        .cloned();

    _ = subscription
        .sender
        .blocking_send(EcsEvents::Subscriber(SubscriptionEvents::Renderer(
            RenderSubscription::OnWorldDidUpdate(view),
        )));
}
