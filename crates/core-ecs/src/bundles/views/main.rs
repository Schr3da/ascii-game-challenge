use core_dtos::prelude::*;

pub fn main_view() -> UiView {
    UiView {
        id: UiViewIds::Main,
        state: UiViewState::default(),
        children: vec![
            UiViewChild::Label(UiLabel {
                id: "title".to_string(),
                text: "Ascii game challenge".to_string(),
            }),
            UiViewChild::List(UiList {
                id: "main-menu-list".to_string(),
                children: vec![],
            }),
        ],
    }
}
