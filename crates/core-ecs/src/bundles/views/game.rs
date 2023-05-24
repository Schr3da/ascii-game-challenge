use core_dtos::prelude::*;

pub fn game_view() -> UiView {
    UiView {
        id: UiViewIds::Game,
        state: UiViewState {
            selected_id: ViewComponentIds::Game(InGameIds::None),
            selectable_ids: Vec::new(),
        },
        children: vec![UiViewChild::Label(UiLabel {
            id: ViewComponentIds::Game(InGameIds::Title),
            text: InGameIds::Title.to_string(),
        })],
    }
}
