use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_click_system(mut store: ResMut<UiStore>, subscriber: Res<Subscriber>) {
    let next = match &subscriber.next_event {
        Some(SendEvents::Ui(UiEvents::OnClick(id))) => id,
        _ => return,
    };

    match next {
        ViewComponentIds::Main(MainMenuIds::NewGame) => {
            store.current_view = UiViewIds::Game;
        }
        ViewComponentIds::Main(MainMenuIds::Options) => {
            store.current_view = UiViewIds::Options;
        }
        ViewComponentIds::Main(MainMenuIds::Quit) => {
            _ = subscriber
                .sender
                .blocking_send(EcsEvents::Send(SendEvents::General(
                    GeneralEvents::OnApplicationWillClose,
                )));
        }
        _ => return,
    };
}
