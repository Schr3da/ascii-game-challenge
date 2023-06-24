use std::io::Stdout;
use tokio::sync::mpsc::error::TryRecvError;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use core_dtos::prelude::*;
use core_state::prelude::*;

use crate::export::prelude::*;

async fn handle_general(event: GeneralSubscription, state: &mut AppState) -> bool {
    match event {
        GeneralSubscription::OnApplicationDidStart => true,
        GeneralSubscription::OnApplicationDidClose => false,
        GeneralSubscription::OnApplicationDidInitialise => {
            let next = SendEvents::Renderer(RenderEvents::OnWorldWillUpdate);
            state.send(next).await;
            return true;
        }
    }
}

async fn handle_ui(_event: UiSubscription) -> bool {
    true
}

async fn handle_renderer(
    event: RenderSubscription,
    app_state: &mut AppState,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> bool {
    match event {
        RenderSubscription::OnWorldDidUpdate(v, p) => {
            app_state.ecs_current_view_state = match &v {
                Some(UiView { state, .. }) => Some(state.clone()),
                _ => None,
            };

            app_state.ecs_current_popup_state = match &p {
                Some(UiView{ state, .. }) => Some(state.clone()),
                _ => None,
            };

            _ = terminal.draw(|f| {
                draw_view_to_terminal_handler(f, &v);
                draw_view_to_terminal_handler(f, &p);
            });
        }
    };

    true
}

async fn handle_command(event: CommandSubscription, app_state: &mut AppState) -> bool {
    match event {
        CommandSubscription::OnCommandDidUpdate(c) => app_state.ecs_current_command = c,
    };

    true
}

pub async fn subscription_handler(
    event: Result<SubscriptionEvents, TryRecvError>,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    state: &mut AppState,
) -> bool {
    let unwrapped_event = match event {
        Ok(e) => e,
        Err(_) => return true,
    };

    match unwrapped_event {
        SubscriptionEvents::General(e) => handle_general(e, state).await,
        SubscriptionEvents::Ui(e) => handle_ui(e).await,
        SubscriptionEvents::Renderer(e) => handle_renderer(e, state, terminal).await,
        SubscriptionEvents::Command(e) => handle_command(e, state).await,
    }
}
