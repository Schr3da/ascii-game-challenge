use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_command_did_update_system(subscription: Res<Subscriber>, state: Res<CommandState>) {
    _ = subscription
        .sender
        .blocking_send(EcsEvents::Subscriber(SubscriptionEvents::Command(
            CommandSubscription::OnCommandDidUpdate(state.current_inputs.clone()),
        )));

    _ = subscription
        .sender
        .blocking_send(EcsEvents::Send(SendEvents::Renderer(
            RenderEvents::OnWorldWillUpdate,
        )));
}
