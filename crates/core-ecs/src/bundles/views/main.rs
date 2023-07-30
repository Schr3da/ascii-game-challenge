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
            selectable_ids: MainMenuIds::get_selectable_items(),
            ..UiViewState::default()
        },
        children: MainMenuIds::get_ui_items(),
    }
}
