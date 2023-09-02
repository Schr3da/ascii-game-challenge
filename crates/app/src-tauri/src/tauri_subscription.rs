use tauri::Window;

use core_dtos::prelude::*;
use core_state::prelude::*;

use crate::export::prelude::*;

pub struct TauriSubscriptions;

impl TauriSubscriptions {
    async fn general(event: &GeneralSubscription, state: &mut AppState, window: &Window) -> bool {
        match event {
            GeneralSubscription::OnApplicationDidStart => ApplicationDispatcher::did_start(),
            GeneralSubscription::OnApplicationDidClose => ApplicationDispatcher::did_close(),
            GeneralSubscription::OnApplicationDidLoadAssets(_) => {
                ApplicationDispatcher::assets_did_load(event, window)
            }
            GeneralSubscription::OnApplicationDidInitialise => {
                ApplicationDispatcher::did_initialise(state).await
            }
        }
    }

    async fn ui(_event: &UiSubscription) -> bool {
        true
    }

    async fn renderer(
        event: &RenderSubscription,
        app_state: &mut AppState,
        window: &Window,
    ) -> bool {
        match event {
            RenderSubscription::OnWorldDidUpdate(v, p, m) => {
                ViewDispatcher::send_view_data(v, app_state, window);
                PopupDispatcher::send_popup_data(p, app_state, window);
                GameDispatcher::send_meta_data(m, app_state, window);
            }
        };

        true
    }

    pub async fn listen(event: SubscriptionEvents, state: &mut AppState, window: &Window) -> bool {
        match &event {
            SubscriptionEvents::General(e) => Self::general(e, state, window).await,
            SubscriptionEvents::Ui(e) => Self::ui(e).await,
            SubscriptionEvents::Renderer(e) => Self::renderer(e, state, window).await,
        }
    }
}
