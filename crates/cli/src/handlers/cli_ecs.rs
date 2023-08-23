use std::io::Stdout;
use tokio::sync::mpsc::error::TryRecvError;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use core_dtos::prelude::*;
use core_state::prelude::*;

use crate::export::prelude::*;

pub struct CliEcsHandler;

impl CliEcsHandler {
    async fn handle_general(event: GeneralSubscription, state: &mut AppState) -> bool {
        match event {
            GeneralSubscription::OnApplicationDidStart => true,
            GeneralSubscription::OnApplicationDidClose => false,
            GeneralSubscription::OnApplicationDidLoadAssets(_) => true,
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
            RenderSubscription::OnWorldDidUpdate(v, p, m) => {
                app_state.ecs_current_view_state = match &v {
                    Some(UiView { state, .. }) => Some(state.clone()),
                    _ => None,
                };

                app_state.ecs_current_popup_state = match &p {
                    Some(UiView { state, .. }) => Some(state.clone()),
                    _ => None,
                };

                app_state.ecs_current_game_status = m.status;

                _ = terminal.draw(|f| {
                    DrawUtils::draw_view(f, &v);
                    DrawUtils::draw_cursor(f, &m.cursor);
                    DrawUtils::draw_view(f, &p);
                });
            }
        };

        true
    }

    pub async fn handle(
        event: Result<SubscriptionEvents, TryRecvError>,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        state: &mut AppState,
    ) -> bool {
        let unwrapped_event = match event {
            Ok(e) => e,
            Err(_) => return true,
        };

        match unwrapped_event {
            SubscriptionEvents::General(e) => Self::handle_general(e, state).await,
            SubscriptionEvents::Ui(e) => Self::handle_ui(e).await,
            SubscriptionEvents::Renderer(e) => Self::handle_renderer(e, state, terminal).await,
        }
    }
}
