use std::time::Duration;

use tauri::Window;
use tokio::{sync::mpsc::*, time::sleep};

use core_state::prelude::*;

use crate::export::prelude::*;

pub fn run(window: Window, js_receiver: Receiver<WebViewEvents>) {
    tauri::async_runtime::spawn(async move {
        let mut webview_event_receiver = js_receiver;

        let mut state = AppState::default();
        state.subscribe();

        loop {
            let webview_event = webview_event_receiver.recv().await;
            let init_done = webview_init_handler(webview_event, &mut state, &window).await;
            if init_done {
                break;
            }
        }

        loop {
            let subscription_event = state.ecs_subscription_receiver.try_recv();
            let should_continue =
                subscription_handler(subscription_event.ok(), &mut state, &window).await;
            if !should_continue {
                _ = window.close();
                break;
            }

            let webview_event = webview_event_receiver.try_recv();
            let did_receive_webview_event =
                webview_event_handler(webview_event.ok(), &mut state).await;

            if !did_receive_webview_event {
                sleep(Duration::from_millis(16)).await;
            }
        }
    });
}
