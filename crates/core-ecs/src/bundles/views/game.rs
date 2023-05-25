use core_dtos::prelude::*;

pub fn game_view() -> UiView {
    UiView {
        id: UiViewIds::Game,
        layout: UiLayout {
            margin: 0,
            alignment: LayoutAlignments::Vertical,
            constraints: vec![LayoutConstraints::Percentage(100)],
        },
        state: UiViewState {
            selected_id: ViewComponentIds::Game(GameIds::None),
            selectable_ids: Vec::new(),
        },
        children: vec![UiViewChild::Section(UiView {
            id: UiViewIds::Game,
            layout: UiLayout {
                margin: 1,
                alignment: LayoutAlignments::Vertical,
                constraints: vec![
                    LayoutConstraints::Percentage(25),
                    LayoutConstraints::Percentage(75),
                ],
            },
            state: UiViewState {
                selected_id: ViewComponentIds::Game(GameIds::None),
                selectable_ids: Vec::new(),
            },
            children: vec![
                UiViewChild::Label(UiLabel {
                    id: ViewComponentIds::Game(GameIds::Title),
                    text: GameIds::Title.to_string(),
                }),
                UiViewChild::Label(UiLabel {
                    id: ViewComponentIds::Game(GameIds::Title),
                    text: GameIds::Title.to_string(),
                }),
            ],
        })],
    }
}
