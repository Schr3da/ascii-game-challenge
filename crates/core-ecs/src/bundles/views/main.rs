use core_dtos::prelude::*;

pub fn main_view() -> UiView {
    UiView {
        id: UiViewIds::Main,
        layout: UiLayout {
            margin: 1,
            alignment: LayoutAlignments::Vertical,
            constraints: vec![
                LayoutConstraints::Percentage(20),
                LayoutConstraints::Percentage(60),
                LayoutConstraints::Percentage(20),
            ],
        },
        state: UiViewState {
            selected_id: ViewComponentIds::Main(MainMenuIds::Quit),
            selectable_ids: vec![
                ViewComponentIds::Main(MainMenuIds::NewGame),
                ViewComponentIds::Main(MainMenuIds::Options),
                ViewComponentIds::Main(MainMenuIds::Quit),
            ],
        },
        children: vec![
            UiViewChild::Label(UiLabel {
                id: ViewComponentIds::Main(MainMenuIds::Title),
                text: "Ascii game challenge".to_string(),
            }),
            UiViewChild::List(UiList {
                id: ViewComponentIds::Main(MainMenuIds::MenuList),
                label: "Main Menu".to_string(),
                children: vec![
                    UiLabel {
                        id: ViewComponentIds::Main(MainMenuIds::NewGame),
                        text: "New Game".to_string(),
                    },
                    UiLabel {
                        id: ViewComponentIds::Main(MainMenuIds::Options),
                        text: "Options".to_string(),
                    },
                    UiLabel {
                        id: ViewComponentIds::Main(MainMenuIds::Quit),
                        text: "Quit".to_string(),
                    },
                ],
            }),
        ],
    }
}
