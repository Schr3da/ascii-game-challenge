use tauri::{Manager, Window};

use core_dtos::prelude::*;
use core_state::prelude::*;

async fn handle_general(event: GeneralSubscription, state: &mut AppState) -> bool {
    match event {
        GeneralSubscription::OnApplicationDidClose => false,
        GeneralSubscription::OnApplicationDidInitialise => {
            let next = SendEvents::Renderer(RenderEvents::OnWorldWillUpdate);
            state.send(next).await;
            true
        }
    }
}

async fn handle_ui(_event: UiSubscription) -> bool {
    true
}

async fn handle_renderer(
    event: RenderSubscription,
    app_state: &mut AppState,
    window: &Window,
) -> bool {
    match event {
        RenderSubscription::OnWorldDidUpdate(v) => {
            let view = match &v {
                Some(v) => v,
                _ => return true,
            };

            app_state.ecs_current_view_state = Some(view.state.clone());

            window.emit_all("OnWorldDidUpdate", view).unwrap();
        }
    };

    true
}

pub async fn subscription_handler(
    event: Option<SubscriptionEvents>,
    state: &mut AppState,
    window: &Window,
) -> bool {
    let unwrapped_event = match event {
        Some(e) => e,
        None => return true,
    };

    match unwrapped_event {
        SubscriptionEvents::General(e) => handle_general(e, state).await,
        SubscriptionEvents::Ui(e) => handle_ui(e).await,
        SubscriptionEvents::Renderer(e) => handle_renderer(e, state, window).await,
    }
}
