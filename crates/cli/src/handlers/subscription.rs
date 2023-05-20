use std::io::Stdout;
use tokio::sync::mpsc::error::TryRecvError;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use core_dtos::prelude::*;
use core_state::prelude::*;

use crate::export::prelude::*;

async fn handle_general(event: GeneralSubscription, state: &mut AppState) -> bool {
    match event {
        GeneralSubscription::OnApplicationDidInitialise => {
            state
                .send(SendEvents::Renderer(RenderEvents::OnWorldWillUpdate))
                .await;

            return true;
        }
        GeneralSubscription::OnApplicationDidClose => false,
    }
}

async fn handle_ui(_event: UiSubscription) -> bool {
    true
}

async fn handle_renderer(
    event: RenderSubscription,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> bool {
    match event {
        RenderSubscription::OnWorldDidUpdate(v) => draw_to_terminal_handler(v, terminal),
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
        SubscriptionEvents::Renderer(e) => handle_renderer(e, terminal).await,
    }
}
