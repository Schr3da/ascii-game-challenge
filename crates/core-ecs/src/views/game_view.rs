use core_dtos::prelude::*;

pub struct GameView;

impl GameView {
    fn top_bar() -> UiViewChild {
        UiViewChild::Section(UiView {
            id: UiViewIds::Game,
            layout: UiLayout {
                margin: 0,
                alignment: LayoutAlignments::Horizontal,
                constraints: vec![
                    LayoutConstraints::Percentage(20),
                    LayoutConstraints::Percentage(60),
                    LayoutConstraints::Percentage(20),
                ],
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
                    shortcut: GameIds::Time.get_shortcut(),
                }),
                UiViewChild::Label(UiLabel {
                    id: ViewComponentIds::Game(GameIds::Day),
                    text: GameIds::Time.to_string(),
                    alignment: TextAlignment::Center,
                    shortcut: GameIds::Day.get_shortcut(),
                }),
                UiViewChild::Label(UiLabel {
                    id: ViewComponentIds::Game(GameIds::Turns),
                    text: GameIds::Turns.to_string(),
                    alignment: TextAlignment::Right,
                    shortcut: GameIds::Turns.get_shortcut(),
                }),
            ],
        })
    }

    fn bottom_bar() -> UiViewChild {
        UiViewChild::Section(UiView {
            id: UiViewIds::Game,
            layout: UiLayout {
                margin: 0,
                alignment: LayoutAlignments::Horizontal,
                constraints: vec![
                    LayoutConstraints::Percentage(50),
                    LayoutConstraints::Percentage(50),
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
                    shortcut: GameIds::Menu.get_shortcut(),
                }),
                UiViewChild::Label(UiLabel {
                    id: ViewComponentIds::Game(GameIds::Actions),
                    text: GameIds::Actions.to_string(),
                    alignment: TextAlignment::Left,
                    shortcut: GameIds::Actions.get_shortcut(),
                }),
            ],
        })
    }

    pub fn new() -> UiView {
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
                Self::top_bar(),
                UiViewChild::GameCanvas(Vec::new()),
                Self::bottom_bar(),
            ],
        }
    }
}
