use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_select_by_id_system(
    store: Res<UiStore>,
    subscriber: Res<Subscriber>,
    mut views_query: Query<&mut UiView>,
) {
    let id = match &subscriber.next_event {
        Some(SendEvents::Ui(UiEvents::OnSelectById(id))) => id,
        _ => return,
    };

    let mut view = match views_query.iter_mut().find(|v| v.id == store.current_view) {
        Some(v) => v,
        None => return,
    };

    view.state.selected_id = id.clone();
}
