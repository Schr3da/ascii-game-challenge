use std::time::Duration;
use tauri::Window;
use tokio::sync::mpsc::*;
use tokio::time::sleep;

use core_state::prelude::*;

use crate::export::prelude::*;

pub fn run(window: Window, mut js_bridge: Receiver<TauriWebViewEvents>) {
    tauri::async_runtime::spawn(async move {
        let mut state = AppState::default();
        state.subscribe();

        let initial_webview_event = match js_bridge.recv().await {
            Some(e) => e,
            None => panic!("Unable to receive initial webview event"),
        };

        TauriWebview::webview_event_handler(initial_webview_event, &mut state).await;

        loop {
            let is_continue = match state.ecs_subscription_receiver.try_recv() {
                Ok(e) => TauriSubscriptions::listen(e, &mut state, &window).await,
                Err(_) => true,
            };

            if !is_continue {
                _ = window.close();
                break;
            }

            let did_receive_input = match js_bridge.try_recv() {
                Ok(e) => TauriWebview::webview_event_handler(e, &mut state).await,
                Err(_) => false,
            };

            if !did_receive_input {
                sleep(Duration::from_millis(16)).await;
            }
        }
    });
}
