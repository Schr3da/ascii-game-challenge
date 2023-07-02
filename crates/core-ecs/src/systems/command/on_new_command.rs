use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_new_command_system(
    subscriber: Res<Subscriber>,
    mut store: ResMut<UiStore>,
    mut views: Query<&mut UiView>,
) {
    match subscriber.next_event {
        Some(SendEvents::Commands(CommandInputEvents::New)) => {}
        _ => return,
    };

    let selected_tile = match &store.selected_game_tile {
        Some(t) => t,
        None => return,
    };

    let mut view = match views
        .iter_mut()
        .find(|v| v.id == UiViewIds::Popup(UiPopupViewIds::Command))
    {
        Some(v) => v,
        None => return,
    };

    view.state.view_data = selected_tile.into();

    store.current_popup = Some(UiPopupViewIds::Command);
}
