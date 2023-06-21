use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_application_will_initialise_system(
    subscriber: Res<Subscriber>,
    mut assets: ResMut<AssetResources>,
    mut store: ResMut<UiStore>,
    mut view_query: Query<&mut UiView>,
    mut popup_query: Query<&mut UiPopupView>,
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

    for mut p in popup_query.iter_mut() {
        p.state = match p.id {
            UiPopupViewIds::Command => UiPopupState {
                selected_id: ViewComponentIds::Popup(CommandPopupIds::UnknownCommandPopupId),
                selectable_ids: vec![],
            },
        }
    }
}
