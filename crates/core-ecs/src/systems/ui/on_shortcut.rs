use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

use crate::prelude::*;

fn get_matching_label(shortcut: &String, component: &UiLabel) -> Option<UiLabel> {
    let next = match &component.shortcut {
        Some(c) => c == shortcut,
        None => return None,
    };

    match next {
        true => Some(component.clone()),
        false => None,
    }
}

fn get_matching_label_in<'a>(shortcut: &String, labels: &Vec<UiLabel>) -> Option<UiLabel> {
    let next = labels
        .iter()
        .find(|l| get_matching_label(shortcut, l).is_some());
    next.cloned()
}

fn find_shortcut_recursive(shortcut: &String, children: &Vec<UiViewChild>) -> Option<UiLabel> {
    let inital_value: Option<UiLabel> = None;

    children.iter().fold(inital_value, |result, c| {
        if result.is_some() {
            return result;
        }

        match &c {
            UiViewChild::List(l) => get_matching_label_in(shortcut, &l.children),
            UiViewChild::Label(l) => get_matching_label(shortcut, l),
            UiViewChild::Section(s) => find_shortcut_recursive(shortcut, &s.children),
            _ => result,
        }
    })
}

fn find_shortcut_for_view(
    shortcut: &String,
    id: &UiViewIds,
    views: &Query<&mut UiView>,
) -> Option<UiLabel> {
    let view = match views.iter().find(|v| &v.id == id) {
        Some(v) => v,
        None => return None,
    };

    find_shortcut_recursive(shortcut, &view.children)
}

fn find_shortcut_for_popup(
    shortcut: &String,
    id: &UiPopupViewIds,
    views: &Query<&mut UiView>,
) -> Option<UiLabel> {
    let id_to_compare = UiViewIds::Popup(id.clone());
    let view = match views.iter().find(|v| &v.id == &id_to_compare) {
        Some(v) => v,
        None => return None,
    };

    find_shortcut_recursive(shortcut, &view.children)
}

pub fn on_shortcut_system(
    mut store: ResMut<UiStore>,
    subscriber: Res<Subscriber>,
    views: Query<&mut UiView>,
) {
    let shortcut = match &subscriber.next_event {
        Some(SendEvents::Ui(UiEvents::OnShortcut(s))) => s,
        _ => return,
    };

    let next = match &store.current_popup {
        Some(id) => find_shortcut_for_popup(shortcut, id, &views),
        None => find_shortcut_for_view(shortcut, &store.current_view, &views),
    };

    let label = match next {
        Some(n) => n,
        None => return,
    };

    if store.current_popup.is_some() {
        return match label.id.to_popup_route() {
            Some(p) => set_next_popup(p, store, views),
            None => return,
        };
    }

    set_next_view(label.id, &mut store, &subscriber);
}
