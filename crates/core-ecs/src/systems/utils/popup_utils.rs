use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

use crate::prelude::*;

pub fn set_next_popup(
    id: UiPopupViewIds,
    mut store: ResMut<UiStore>,
    logger: ResMut<Logger>,
    mut views: Query<&mut UiView>,
) {
    let selected_tile = match &store.selected_game_tile {
        Some(t) => t,
        None => return,
    };

    let id_to_compare = UiViewIds::Popup(id.clone());
    let mut view = match views.iter_mut().find(|v| v.id == id_to_compare) {
        Some(v) => v,
        None => return,
    };

    view.state.view_data = match id_to_compare {
        UiViewIds::Popup(UiPopupViewIds::Logger) => logger.as_ref().into(),
        _ => selected_tile.into(),
    };

    store.current_popup = Some(id);
}
