use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

use crate::prelude::*;

pub struct ApplicationSystems;

impl ApplicationSystems {
    pub fn on_application_will_close(subscription: Res<SubscriberResource>) {
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

    pub fn on_application_will_resize(
        mut store: ResMut<UiStoreResource>,
        mut camera: ResMut<CameraResource>,
        subscription: ResMut<SubscriberResource>,
    ) {
        let (width, height) = match subscription.next_event {
            Some(SendEvents::General(GeneralEvents::OnApplicationResize(w, h))) => {
                (w as i32, h as i32)
            }
            _ => return,
        };

        if let Some(t) = &mut store.selected_game_tile {
            t.handle_window_resize(width, height);
        }

        camera.viewport = Rect {
            width,
            height,
            ..camera.viewport
        };

        _ = subscription
            .sender
            .blocking_send(EcsEvents::Send(SendEvents::Renderer(
                RenderEvents::OnWorldWillUpdate,
            )));
    }
}
