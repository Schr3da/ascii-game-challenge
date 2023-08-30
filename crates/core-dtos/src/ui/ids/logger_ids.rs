use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Tsify)]
pub enum LoggerIds {
    Print,
}

impl ToShortcut for LoggerIds {
    fn get_shortcut(&self) -> Option<String> {
      None
    }
}

impl ToSelectable for LoggerIds{
    type Item = ViewComponentIds;

    fn get_selectable_items() -> Vec<Self::Item> {
        vec![]
    }
}

impl ToUiViewChildren for LoggerIds {
    fn get_ui_items() -> Vec<UiViewChild> {
        vec![UiViewChild::List(UiList {
            id: ViewComponentIds::Popup(PopupIds::Log(None)),
            label: "Available Logs".to_string(),
            children: vec![UiLabel {
                id: ViewComponentIds::Popup(PopupIds::Log(Some(LoggerIds::Print))),
                alignment: TextAlignment::Left,
                text: "".to_string(),
                shortcut: None, 
            }],
        })]
    }
}
