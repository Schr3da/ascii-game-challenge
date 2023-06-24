use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_application_will_initialise_system(
    subscriber: Res<Subscriber>,
    mut assets: ResMut<AssetResources>,
    mut store: ResMut<UiStore>,
    mut view_query: Query<&mut UiView>,
) {
    let (width, height) = match subscriber.next_event {
        Some(SendEvents::General(GeneralEvents::OnApplicationWillInitialise(w, h))) => (w, h),
        _ => (0, 0),
    };

    store.width = width;
    store.height = height;

    store.previous_view = vec![];
    store.current_view = UiViewIds::Main;

    assets.terrain.generate();

    for mut v in view_query.iter_mut() {
        v.state = match &v.id {
            UiViewIds::Main => UiViewState {
                selected_id: ViewComponentIds::Main(MainMenuIds::NewGame),
                selectable_ids: MainMenuIds::get_selectable_items(),
            },
            UiViewIds::Options => UiViewState {
                selected_id: ViewComponentIds::Options(OptionMenuIds::LevelOfDifficulty),
                selectable_ids: OptionMenuIds::get_selectable_items(),
            },
            UiViewIds::Game => UiViewState {
                selected_id: ViewComponentIds::Game(GameIds::None),
                selectable_ids: GameIds::get_selectable_items(),
            },
            UiViewIds::Popup(id) => match id {
                UiPopupViewIds::Command => UiViewState {
                    selected_id: ViewComponentIds::CommandPopup(CommandIds::Move),
                    selectable_ids: CommandIds::get_selectable_items(),
                },
            },
        };
    }
}
