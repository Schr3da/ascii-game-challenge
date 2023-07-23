use tauri::Window;

use core_dtos::prelude::*;
use core_state::prelude::*;

use crate::export::prelude::*;

async fn handle_general(
    event: &GeneralSubscription,
    state: &mut AppState,
    window: &Window,
) -> bool {
    match event {
        GeneralSubscription::OnApplicationDidStart => dispatch_application_did_start(),
        GeneralSubscription::OnApplicationDidClose => dispatch_application_did_close(),
        GeneralSubscription::OnApplicationDidLoadAssets(_) => {
            dispatch_dispatch_assets_did_load(event, window)
        }
        GeneralSubscription::OnApplicationDidInitialise => {
            dispatch_appliaction_did_initialise(state).await
        }
    }
}

async fn handle_ui(_event: &UiSubscription) -> bool {
    true
}

async fn handle_renderer(
    event: &RenderSubscription,
    app_state: &mut AppState,
    window: &Window,
) -> bool {
    match event {
        RenderSubscription::OnWorldDidUpdate(v, p, m) => {
            if let Some(n) = v {
                dispatch_view_data(n, app_state, window);
            }

            if let Some(n) = p {
                dispatch_popup_data(n, app_state, window);
            }

            dispatch_game_meta(m, app_state, window);
        }
    };

    true
}

pub async fn handle_command(event: &CommandSubscription, app_state: &mut AppState) -> bool {
    match event {
        CommandSubscription::OnCommandDidUpdate(c) => app_state.ecs_current_command = c.clone(),
    };

    true
}

pub async fn subscription_handler(
    event: Option<SubscriptionEvents>,
    state: &mut AppState,
    window: &Window,
) -> bool {
    let unwrapped_event = match &event {
        Some(e) => e,
        None => return true,
    };

    match unwrapped_event {
        SubscriptionEvents::General(e) => handle_general(e, state, window).await,
        SubscriptionEvents::Ui(e) => handle_ui(e).await,
        SubscriptionEvents::Renderer(e) => handle_renderer(e, state, window).await,
        SubscriptionEvents::Command(e) => handle_command(e, state).await,
    }
}
