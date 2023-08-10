use core_inputs::prelude::{handle_keyboard_event, handle_mouse_event};

use core_dtos::prelude::*;
use core_state::prelude::*;

use crate::export::prelude::*;

async fn handle_did_mount(columns: u16, rows: u16, app_state: &mut AppState) -> bool {
    let next = SendEvents::General(GeneralEvents::OnApplicationWillInitialise(columns, rows));

    _ = app_state.send(next).await;
    true
}

async fn handle_input_event(event: InputEvents, app_state: &mut AppState) -> bool {
    match event {
        InputEvents::Mouse(e) => handle_mouse_event(e, app_state).await,
        InputEvents::Keyboard(KeyboardEvent::OnPress(k)) => {
            handle_keyboard_event(k, app_state).await
        }
    };
    true
}

async fn handle_ecs_event(event: SendEvents, app_state: &mut AppState) -> bool {
    app_state.send(event).await;
    true
}

pub async fn webview_event_handler(event: WebViewEvents, app_state: &mut AppState) -> bool {
    match event {
        WebViewEvents::OnEcsEvent(e) => handle_ecs_event(e, app_state).await,
        WebViewEvents::OnInputEvent(e) => handle_input_event(e, app_state).await,
        WebViewEvents::OnDidMount(c, r) => handle_did_mount(c, r, app_state).await,
        _ => false,
    }
}
