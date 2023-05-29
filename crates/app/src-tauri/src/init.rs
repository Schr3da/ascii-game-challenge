use tauri::Window;
use tokio::sync::mpsc::*;

use core_dtos::prelude::*;
use core_state::prelude::*;

use crate::subscription::*;

pub fn run(window: Window, js_receiver: Receiver<String>) {
    tauri::async_runtime::spawn(async move {
        let mut wait_for_init = js_receiver;
        wait_for_init.recv().await;

        let size = window.inner_size().unwrap_or_default();

        let mut state = AppState::default();
        state.subscribe();

        state
            .send(SendEvents::General(
                GeneralEvents::OnApplicationWillInitialise(size.width as u16, size.height as u16),
            ))
            .await;

        loop {
            let subscription_event = state.ecs_subscription_receiver.recv().await;
            let will_exit = subscription_handler(subscription_event, &mut state, &window).await;
            if !will_exit {
                break;
            }
        }
    });
}
