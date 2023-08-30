use bevy_ecs::prelude::*;
use core_dtos::prelude::*;
use core_formatters::prelude::DigitFormatter;

use crate::prelude::*;

pub struct TickSystems;

impl TickSystems {
    fn update_game_time(data: &mut GameViewHeaderState, clock: &mut ResMut<ClockResource>) {
        clock.update();

        data.tick_count = clock.ticks.to_string();
        data.current_days = clock.days.to_string();
        data.current_hours = DigitFormatter::prettify_i32(clock.hours);
        data.current_minutes = DigitFormatter::prettify_i32(clock.minutes);
    }

    pub fn on_update_view_labels(
        store: Res<UiStoreResource>,
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
                    Self::update_game_time(d, &mut clock);
                }

                v.state.view_data = view_data;
            }
            _ => return,
        };
    }

    pub fn on_tick_did_complete(subscription: Res<SubscriberResource>) {
        _ = subscription
            .sender
            .blocking_send(EcsEvents::Send(SendEvents::Renderer(
                RenderEvents::OnWorldWillUpdate,
            )));
    }
}
