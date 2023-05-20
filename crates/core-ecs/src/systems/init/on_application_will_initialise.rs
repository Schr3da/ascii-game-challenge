use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_application_will_initialise_system(
    mut store: ResMut<UiStore>,
    mut query: Query<&mut UiView>,
) {
    store.current_view = UiViewIds::Main;

    for mut v in query.iter_mut() {
        v.state = UiViewState::default();
    }
}
