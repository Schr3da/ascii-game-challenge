use core_dtos::prelude::*;

pub fn get_top_bar() -> UiViewChild {
    UiViewChild::Section(UiView {
        id: UiViewIds::Game,
        layout: UiLayout {
            margin: 0,
            alignment: LayoutAlignments::Horizontal,
            constraints: vec![LayoutConstraints::Percentage(20), LayoutConstraints::Percentage(60), LayoutConstraints::Percentage(20)],
        },
        state: UiViewState {
            selected_id: ViewComponentIds::Game(GameIds::None),
            selectable_ids: Vec::new(),
            view_data: ViewDataTypes::GameHeader(GameViewHeaderState::default()),
            ..UiViewState::default()
        },
        children: vec![
            UiViewChild::Label(UiLabel {
                id: ViewComponentIds::Game(GameIds::Time),
                text: GameIds::Time.to_string(),
                alignment: TextAlignment::Left,
            }),
            UiViewChild::Label(UiLabel {
                id: ViewComponentIds::Game(GameIds::Day),
                text: GameIds::Time.to_string(),
                alignment: TextAlignment::Center,
            }),
            UiViewChild::Label(UiLabel {
                id: ViewComponentIds::Game(GameIds::Turns),
                text: GameIds::Turns.to_string(),
                alignment: TextAlignment::Right,
            }),
        ],
    })
}

pub fn get_bottom_bar() -> UiViewChild {
    UiViewChild::Section(UiView {
        id: UiViewIds::Game,
        layout: UiLayout {
            margin: 0,
            alignment: LayoutAlignments::Horizontal,
            constraints: vec![
                LayoutConstraints::Percentage(40),
                LayoutConstraints::Percentage(20),
                LayoutConstraints::Percentage(20),
                LayoutConstraints::Percentage(20),
            ],
        },
        state: UiViewState {
            selected_id: ViewComponentIds::Game(GameIds::None),
            selectable_ids: Vec::new(),
            ..UiViewState::default()
        },
        children: vec![
            UiViewChild::Label(UiLabel {
                id: ViewComponentIds::Game(GameIds::Menu),
                text: GameIds::Menu.to_string(),
                alignment: TextAlignment::Left,
            }),
            UiViewChild::Label(UiLabel {
                id: ViewComponentIds::Game(GameIds::Food),
                text: GameIds::Food.to_string(),
                alignment: TextAlignment::Left,
            }),
            UiViewChild::Label(UiLabel {
                id: ViewComponentIds::Game(GameIds::Wood),
                text: GameIds::Wood.to_string(),
                alignment: TextAlignment::Left,
            }),
            UiViewChild::Label(UiLabel {
                id: ViewComponentIds::Game(GameIds::Stones),
                text: GameIds::Stones.to_string(),

                alignment: TextAlignment::Left,
            }),
        ],
    })
}

pub fn game_view() -> UiView {
    UiView {
        id: UiViewIds::Game,
        layout: UiLayout {
            margin: 0,
            alignment: LayoutAlignments::Vertical,
            constraints: vec![
                LayoutConstraints::Value(1),
                LayoutConstraints::MaxValue(100),
                LayoutConstraints::Value(1),
            ],
        },
        state: UiViewState {
            selected_id: ViewComponentIds::Game(GameIds::None),
            selectable_ids: Vec::new(),
            ..UiViewState::default()
        },
        children: vec![
            get_top_bar(),
            UiViewChild::GameCanvas(Vec::new(), None),
            get_bottom_bar(),
        ],
    }
}
