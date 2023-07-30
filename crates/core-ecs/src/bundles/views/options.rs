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
            selectable_ids: OptionMenuIds::get_selectable_items(),
            ..UiViewState::default()
        },
        children: OptionMenuIds::get_ui_items(),
    }
}
