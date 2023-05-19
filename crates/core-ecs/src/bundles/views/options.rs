use core_dtos::prelude::*;

use crate::prelude::*;

pub fn options_view() -> UiView {
    UiView {
        id: UiViewIds::Options,
        state: UiViewState::default(),
        children: vec![
            UiViewChild::Label(UiLabel {
                id: "title".to_string(),
                text: "Options".to_string(),
            }),
            UiViewChild::List(UiList {
                id: "options-list".to_string(),
                children: vec![],
            }),
        ],
    }
}
