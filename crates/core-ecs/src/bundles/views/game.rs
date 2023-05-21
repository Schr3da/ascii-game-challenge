use core_dtos::prelude::*;

pub fn game_view() -> UiView {
    UiView {
        id: UiViewIds::Game,
        state: UiViewState {
            selected_id: ViewComponentIds::Game(InGameIds::None),
        },
        children: vec![],
    }
}
