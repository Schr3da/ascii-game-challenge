use tauri::Window;

use core_dtos::prelude::*;
use core_state::prelude::*;

use crate::export::prelude::*;

pub async fn webview_init_handler(
    event: Option<WebViewEvents>,
    sender: &EcsSender,
    window: &Window,
) {
    let unwrapped_event = match event {
        Some(e) => e,
        None => return,
    };

    let size = window.inner_size().unwrap_or_default();

    match unwrapped_event {
        WebViewEvents::OnDidSubscribe => {
            let next = SendEvents::General(GeneralEvents::OnApplicationWillInitialise(
                size.width as u16,
                size.height as u16,
            ));

            _ = sender.send(EcsEvents::Send(next)).await;
        }
        _ => {}
    }
}

pub async fn webview_event_handler(event: Option<WebViewEvents>, sender: &EcsSender) {
    let unwrapped_event = match event {
        Some(e) => e,
        None => return,
    };

    match unwrapped_event {
        WebViewEvents::OnEcsEvent(e) => _ = sender.send(EcsEvents::Send(e)).await,
        _ => return,
    };
}
