use tauri::Window;

use core_dtos::prelude::*;
use core_state::prelude::*;

async fn handle_general(event: &GeneralSubscription, state: &mut AppState) -> bool {
    match event {
        GeneralSubscription::OnApplicationDidStart => true,
        GeneralSubscription::OnApplicationDidClose => false,
        GeneralSubscription::OnApplicationDidInitialise => {
            let next = SendEvents::Renderer(RenderEvents::OnWorldWillUpdate);
            state.send(next).await;
            true
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
        RenderSubscription::OnWorldDidUpdate(v, p, s) => {
            app_state.ecs_current_view_state = match &v {
                Some(UiView { state, .. }) => Some(state.clone()),
                _ => None,
            };

            app_state.ecs_current_popup_state = match &p {
                Some(UiView { state, .. }) => Some(state.clone()),
                _ => None,
            };

            app_state.ecs_current_game_status = s.clone();

            if v.is_none() && p.is_none() {
                return true;
            }

            _ = window.emit("ecs-subscription", event);
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
        SubscriptionEvents::General(e) => handle_general(e, state).await,
        SubscriptionEvents::Ui(e) => handle_ui(e).await,
        SubscriptionEvents::Renderer(e) => handle_renderer(e, state, window).await,
        SubscriptionEvents::Command(e) => handle_command(e, state).await,
    }
}
