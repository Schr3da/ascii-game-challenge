use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

fn select_previous(mut view: Mut<UiView>) {
    let ids = &view.state.selectable_ids;
    let id = &view.state.selected_id;
    let current = ids.iter().position(|i| i == id).unwrap_or(0);

    if current == 0 {
        return view.state.selected_id = ids.last().cloned().unwrap_or_default();
    }

    match ids.get(current - 1) {
        Some(n) => view.state.selected_id = n.clone(),
        None => view.state.selected_id = ids.last().cloned().unwrap_or_default(),
    };
}

fn select_next(mut view: Mut<UiView>) {
    let ids = &view.state.selectable_ids;
    let id = &view.state.selected_id;
    let current = ids.iter().position(|i| i == id).unwrap_or(0);

    match ids.get(current + 1) {
        Some(n) => view.state.selected_id = n.clone(),
        None => view.state.selected_id = ids.first().cloned().unwrap_or_default(),
    };
}

pub fn on_select_system(
    store: Res<UiStore>,
    subscriber: Res<Subscriber>,
    mut views_query: Query<&mut UiView>,
) {
    let direction = match &subscriber.next_event {
        Some(SendEvents::Ui(UiEvents::OnSelect(d))) => d,
        _ => return,
    };

    let view = match views_query.iter_mut().find(|v| v.id == store.current_view) {
        Some(v) => v,
        None => return,
    };

    match direction {
        SelectionDirections::Next => select_next(view),
        SelectionDirections::Previous => select_previous(view),
    };
}
