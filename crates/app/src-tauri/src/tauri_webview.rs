use core_inputs::prelude::*;

use core_dtos::prelude::*;
use core_state::prelude::*;

#[derive(Debug)]
pub enum TauriWebViewEvents {
    OnDidMount(u16, u16),
    OnDidSubscribe,
    OnEcsEvent(SendEvents),
    OnInputEvent(InputEvents),
}

pub struct TauriWebview;

impl TauriWebview {
    async fn did_mount(columns: u16, rows: u16, app_state: &mut AppState) -> bool {
        let next = SendEvents::General(GeneralEvents::OnApplicationWillInitialise(columns, rows));

        _ = app_state.send(next).await;
        true
    }

    async fn input_event(event: InputEvents, app_state: &mut AppState) -> bool {
        match event {
            InputEvents::Mouse(e) => {
                MouseInputs::handle_event(e, app_state).await;
            }
            InputEvents::Keyboard(KeyboardEvent::OnPress(k)) => {
                KeyboardInputs::handle_event(k, app_state).await;
            }
        };
        true
    }

    async fn ecs_event(event: SendEvents, app_state: &mut AppState) -> bool {
        app_state.send(event).await;
        true
    }

    pub async fn webview_event_handler(
        event: TauriWebViewEvents,
        app_state: &mut AppState,
    ) -> bool {
        match event {
            TauriWebViewEvents::OnEcsEvent(e) => Self::ecs_event(e, app_state).await,
            TauriWebViewEvents::OnInputEvent(e) => Self::input_event(e, app_state).await,
            TauriWebViewEvents::OnDidMount(c, r) => Self::did_mount(c, r, app_state).await,
            _ => false,
        }
    }
}
