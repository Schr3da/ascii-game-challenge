use core_dtos::prelude::*;

pub fn options_view() -> UiView {
    UiView {
        id: UiViewIds::Options,
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
            selected_id: ViewComponentIds::Options(OptionMenuIds::LevelOfDifficulty),
            selectable_ids: vec![
                ViewComponentIds::Options(OptionMenuIds::LevelOfDifficulty),
                ViewComponentIds::Options(OptionMenuIds::Sound),
                ViewComponentIds::Options(OptionMenuIds::Back),
            ],
        },
        children: vec![
            UiViewChild::Label(UiLabel {
                id: ViewComponentIds::Options(OptionMenuIds::Title),
                text: "".to_string(),
                alignment: TextAlignment::Center,
            }),
            UiViewChild::List(UiList {
                id: ViewComponentIds::Options(OptionMenuIds::OptionList),
                label: "Options".to_string(),
                children: vec![
                    UiLabel {
                        id: ViewComponentIds::Options(OptionMenuIds::LevelOfDifficulty),
                        text: "Level".to_string(),
                        alignment: TextAlignment::Left,
                    },
                    UiLabel {
                        id: ViewComponentIds::Options(OptionMenuIds::Sound),
                        text: "Sound".to_string(),
                        alignment: TextAlignment::Left,
                    },
                    UiLabel {
                        id: ViewComponentIds::Options(OptionMenuIds::Back),
                        text: "Back".to_string(),
                        alignment: TextAlignment::Left,
                    },
                ],
            }),
        ],
    }
}
