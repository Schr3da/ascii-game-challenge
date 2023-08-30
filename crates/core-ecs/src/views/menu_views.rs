use core_dtos::prelude::*;

pub struct MenuViews;

impl MenuViews {
    pub fn main_menu() -> UiView {
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
                selectable_ids: MainMenuIds::get_selectable_items(),
                ..UiViewState::default()
            },
            children: MainMenuIds::get_ui_items(),
        }
    }

    pub fn settings() -> UiView {
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
                selectable_ids: OptionMenuIds::get_selectable_items(),
                ..UiViewState::default()
            },
            children: OptionMenuIds::get_ui_items(),
        }
    }
}
