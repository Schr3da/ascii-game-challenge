use tauri::Window;

use core_dtos::prelude::*;
use core_state::prelude::*;
use tokio::sync::mpsc::error::TryRecvError;

use crate::export::prelude::*;

pub async fn webview_init_handler(
    event: Option<WebViewEvents>,
    state: &mut AppState,
    window: &Window,
) -> bool {
    let unwrapped_event = match event {
        Some(e) => e,
        None => return false,
    };

    let size = window.inner_size().unwrap_or_default();

    match unwrapped_event {
        WebViewEvents::OnDidSubscribe => {
            let next = SendEvents::General(GeneralEvents::OnApplicationWillInitialise(
                size.width as u16,
                size.height as u16,
            ));

            state.send(next).await;
            true
        }
        _ => false,
    }
}

pub async fn webview_event_handler(
    event: Result<WebViewEvents, TryRecvError>,
    state: &mut AppState,
) {
    let unwrapped_event = match event {
        Ok(e) => e,
        Err(_) => return,
    };

    match unwrapped_event {
        WebViewEvents::OnEcsEvent(e) => state.send(e).await,
        _ => return,
    };
}
