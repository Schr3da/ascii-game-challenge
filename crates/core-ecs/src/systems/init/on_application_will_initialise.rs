use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_application_will_initialise_system(
    mut store: ResMut<UiStore>,
    mut query: Query<&mut UiView>,
) {
    store.current_view = UiViewIds::Main;

    for mut v in query.iter_mut() {
        v.state = match v.id {
            UiViewIds::Main => UiViewState {
                selected_id: ViewComponentIds::Main(MainMenuIds::NewGame),
                selectable_ids: MainMenuIds::get_selectable_items(),
            },
            UiViewIds::Options => UiViewState {
                selected_id: ViewComponentIds::Options(OptionMenuIds::Back),
                selectable_ids: OptionMenuIds::get_selectable_items(),
            },
            UiViewIds::Game => UiViewState {
                selected_id: ViewComponentIds::Game(GameIds::None),
                selectable_ids: GameIds::get_selectable_items(),
            },
        };
    }
}
