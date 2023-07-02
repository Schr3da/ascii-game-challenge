use bevy_ecs::prelude::*;

use core_dtos::prelude::*;
use core_formatters::prelude::prettify_i32;

use crate::prelude::*;

fn update_game_time(data: &mut GameViewHeaderState, clock: &mut ResMut<ClockResource>) {
    clock.update();

    data.tick_count = clock.ticks.to_string();
    data.current_days = clock.days.to_string();
    data.current_hours = prettify_i32(clock.hours);
    data.current_minutes = prettify_i32(clock.minutes);
}

pub fn on_update_view_labels_system(
    store: Res<UiStore>,
    mut clock: ResMut<ClockResource>,
    mut views_query: Query<&mut UiView>,
) {
    let current_view = store.current_view.clone();

    let mut view = match views_query.iter_mut().find(|v| v.id == current_view) {
        Some(v) => v,
        None => return,
    };

    let mut header_wrapper = match view.children.first_mut() {
        Some(v) => v,
        None => return,
    };

    match &mut header_wrapper {
        UiViewChild::Section(v) => {
            let mut view_data = v.state.view_data.clone();

            if let ViewDataTypes::GameHeader(d) = &mut view_data {
                update_game_time(d, &mut clock);
            }

            v.state.view_data = view_data;
        }
        _ => return,
    };
}
