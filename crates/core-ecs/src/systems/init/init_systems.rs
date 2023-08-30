use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub struct InitSystems;

impl InitSystems {
    pub fn on_application_will_initialise(
        subscriber: Res<SubscriberResource>,
        mut assets: ResMut<AssetResources>,
        mut store: ResMut<UiStoreResource>,
        mut camera: ResMut<CameraResource>,
        mut view_query: Query<&mut UiView>,
    ) {
        let (width, height) = match subscriber.next_event {
            Some(SendEvents::General(GeneralEvents::OnApplicationWillInitialise(w, h))) => (w, h),
            _ => (0, 0),
        };

        camera.viewport.set_width(width);
        camera.viewport.set_height(height);

        store.previous_view = vec![];
        store.current_view = UiViewIds::Main;

        assets.terrain.generate();

        for mut v in view_query.iter_mut() {
            v.state = match &v.id {
                UiViewIds::Main => UiViewState {
                    selected_id: ViewComponentIds::Main(MainMenuIds::NewGame),
                    selectable_ids: MainMenuIds::get_selectable_items(),
                    ..UiViewState::default()
                },
                UiViewIds::Options => UiViewState {
                    selected_id: ViewComponentIds::Options(OptionMenuIds::LevelOfDifficulty),
                    selectable_ids: OptionMenuIds::get_selectable_items(),
                    ..UiViewState::default()
                },
                UiViewIds::Game => UiViewState {
                    selected_id: ViewComponentIds::Game(GameIds::None),
                    selectable_ids: GameIds::get_selectable_items(),
                    ..UiViewState::default()
                },
                UiViewIds::Popup(id) => match id {
                    UiPopupViewIds::Actions => UiViewState {
                        selected_id: ViewComponentIds::Popup(PopupIds::Build(None)),
                        selectable_ids: PopupIds::get_selectable_items(),
                        ..UiViewState::default()
                    },
                    UiPopupViewIds::Buildings => UiViewState {
                        selected_id: ViewComponentIds::Popup(PopupIds::Build(None)),
                        selectable_ids: BuildingIds::get_selectable_items(),
                        ..UiViewState::default()
                    },
                    UiPopupViewIds::Logs => UiViewState {
                        selected_id: ViewComponentIds::Popup(PopupIds::Log(None)),
                        selectable_ids: vec![],
                        ..UiViewState::default()
                    },
                },
                UiViewIds::Quit => return,
            };
        }
    }

    pub fn on_application_did_initialise(subscription: ResMut<SubscriberResource>) {
        _ = subscription
            .sender
            .blocking_send(EcsEvents::Subscriber(SubscriptionEvents::General(
                GeneralSubscription::OnApplicationDidInitialise,
            )));
    }

    pub fn on_application_will_load_assets(
        subscription: Res<SubscriberResource>,
        assets: Res<AssetResources>,
    ) {
        match subscription.next_event {
            Some(SendEvents::General(GeneralEvents::OnApplicationWillInitialise(_, _))) => {}
            _ => return,
        };

        let cells = assets.cell_cache.clone();

        _ = subscription
            .sender
            .blocking_send(EcsEvents::Subscriber(SubscriptionEvents::General(
                GeneralSubscription::OnApplicationDidLoadAssets(cells),
            )));
    }
}
