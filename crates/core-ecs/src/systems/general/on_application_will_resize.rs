use bevy_ecs::system::ResMut;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_application_will_resize_system(
    mut store: ResMut<UiStore>,
    mut camera: ResMut<Camera2d>,
    subscription: ResMut<Subscriber>,
) {
    let (width, height) = match subscription.next_event {
        Some(SendEvents::General(GeneralEvents::OnApplicationResize(w, h))) => (w as i32, h as i32),
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
