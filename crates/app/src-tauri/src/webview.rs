use core_inputs::prelude::{handle_keyboard_event, handle_mouse_event};
use tauri::Window;

use core_dtos::prelude::*;
use core_state::prelude::*;

use crate::export::prelude::*;

async fn handle_did_subscribe(window: &Window, app_state: &mut AppState) -> bool {
    let size = window.inner_size().unwrap_or_default();

    let next = SendEvents::General(GeneralEvents::OnApplicationWillInitialise(
        size.width as u16,
        size.height as u16,
    ));

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

pub async fn webview_event_handler(
    event: WebViewEvents,
    window: &Window,
    app_state: &mut AppState,
) -> bool {
    match event {
        WebViewEvents::OnEcsEvent(e) => handle_ecs_event(e, app_state).await,
        WebViewEvents::OnInputEvent(e) => handle_input_event(e, app_state).await,
        WebViewEvents::OnDidSubscribe => handle_did_subscribe(window, app_state).await,
        _ => false,
    }
}
