use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_click_system(mut store: ResMut<UiStore>, subscriber: Res<Subscriber>) {
    let next = match &subscriber.next_event {
        Some(SendEvents::Ui(UiEvents::OnClick(id))) => id,
        _ => return,
    };

    let current_view = store.current_view.clone();

    match next {
        ViewComponentIds::Main(MainMenuIds::NewGame) => {
            store.previous_view.push(current_view);
            store.current_view = UiViewIds::Game;
            store.selected_game_tile = Some(SelectedCell::default());
        }
        ViewComponentIds::Main(MainMenuIds::Options) => {
            store.previous_view.push(current_view);
            store.current_view = UiViewIds::Options;
        }
        ViewComponentIds::Main(MainMenuIds::Quit) => {
            _ = subscriber
                .sender
                .blocking_send(EcsEvents::Send(SendEvents::General(
                    GeneralEvents::OnApplicationWillClose,
                )));
        }
        ViewComponentIds::Options(OptionMenuIds::Back) => {
            store.previous_view.push(current_view);
            store.current_view = UiViewIds::Main;
        }
        _ => {
            println!("Click on item not supported {:?}", next);
        }
    };
}
