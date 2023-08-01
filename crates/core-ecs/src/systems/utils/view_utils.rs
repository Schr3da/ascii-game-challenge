use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

use crate::prelude::*;

pub fn set_next_view(
    id: ViewComponentIds,
    store: &mut ResMut<UiStore>,
    subscriber: &Res<Subscriber>,
) {
    let current_view = store.current_view.clone();

    let next_view = match id.to_view_route() {
        Some(n) => n,
        None => return,
    };

    match id {
        ViewComponentIds::Main(MainMenuIds::NewGame) => {
            store.previous_view.push(current_view);
            store.current_view = next_view;
            store.current_game_status = GameStatus::GameDidStart;
            store.selected_game_tile = Some(SelectedCell::default());
        }
        ViewComponentIds::Main(MainMenuIds::Options) => {
            store.previous_view.push(current_view);
            store.current_view = next_view;
            store.current_game_status = GameStatus::GameDidNotStart;
        }
        ViewComponentIds::Options(OptionMenuIds::Back) => {
            store.previous_view.push(current_view);
            store.current_view = next_view;
            store.current_game_status = GameStatus::GameDidNotStart;
        }
        ViewComponentIds::Main(MainMenuIds::Quit) => {
            store.current_game_status = GameStatus::GameWillEnded;
            _ = subscriber
                .sender
                .blocking_send(EcsEvents::Send(SendEvents::General(
                    GeneralEvents::OnApplicationWillClose,
                )));
        }
        _ => {
            println!("Click on item not supported {:?}", next_view);
        }
    }
}
