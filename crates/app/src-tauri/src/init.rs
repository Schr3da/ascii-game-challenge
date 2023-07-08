use tauri::Window;
use tokio::sync::mpsc::*;

use core_state::prelude::*;

use crate::export::prelude::*;

async fn setup_webview_handler(
    window: &Window,
    state: &AppState,
    mut js_receiver: Receiver<WebViewEvents>,
) {
    let webview_event = js_receiver.recv().await;
    let scoped_sender = state.ecs_event_sender.clone();

    tauri::async_runtime::spawn(async move {
        loop {
            let event = js_receiver.recv().await;
            webview_event_handler(event, &scoped_sender).await;
        }
    });

    let sender = state.ecs_event_sender.clone();
    webview_init_handler(webview_event, &sender, &window).await;
}

fn setup_subscription_receiver(window: Window, mut state: AppState) {
    tauri::async_runtime::spawn(async move {
        loop {
            let subscription_event = state.ecs_subscription_receiver.recv().await;
            let is_continue = subscription_handler(subscription_event, &mut state, &window).await;

            if !is_continue {
                _ = window.close();
                break;
            }
        }
    });
}

pub fn run(window: Window, js_receiver: Receiver<WebViewEvents>) {
    tauri::async_runtime::spawn(async move {
        let mut state = AppState::default();
        state.subscribe();

        setup_webview_handler(&window, &state, js_receiver).await;
        setup_subscription_receiver(window, state);
    });
}
