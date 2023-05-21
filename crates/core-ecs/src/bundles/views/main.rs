use core_dtos::prelude::*;

pub fn main_view() -> UiView {
    UiView {
        id: UiViewIds::Main,
        state: UiViewState {
            selected_id: ViewComponentIds::Main(MainMenu::NewGame),
        },
        children: vec![
            UiViewChild::Label(UiLabel {
                id: ViewComponentIds::Main(MainMenu::Title),
                text: "Ascii game challenge".to_string(),
            }),
            UiViewChild::List(UiList {
                id: ViewComponentIds::Main(MainMenu::MenuList),
                children: vec![
                    UiLabel {
                        id: ViewComponentIds::Main(MainMenu::NewGame),
                        text: "New Game".to_string(),
                    },
                    UiLabel {
                        id: ViewComponentIds::Main(MainMenu::Options),
                        text: "Options".to_string(),
                    },
                    UiLabel {
                        id: ViewComponentIds::Main(MainMenu::Quit),
                        text: "Quit".to_string(),
                    },
                ],
            }),
        ],
    }
}
