use std::collections::HashMap;

use core_dtos::prelude::*;

pub fn ui_label_full_title(component: &UiLabel) -> String {
    match &component.shortcut {
        Some(s) => format!("[{}] {}", s, component.text),
        None => format!("{}", component.text),
    }
}

pub fn ui_label_full_title_with(view_data: &ViewDataTypes, component: &UiLabel) -> String {
    let data: HashMap<ViewComponentIds, String> = view_data.into();

    let title = match data.get(&component.id) {
        Some(v) => v,
        None => &component.text,
    };

    match &component.shortcut {
        Some(s) => format!("[{}] {}", s, title),
        None => format!("{}", title),
    }
}
