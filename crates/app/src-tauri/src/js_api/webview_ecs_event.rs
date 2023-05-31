use crate::export::prelude::*;
use core_dtos::prelude::*;

#[tauri::command]
pub async fn webview_ecs_event(
    event: SendEvents,
    state: tauri::State<'_, JsSignal>,
) -> Result<(), ()> {
    let sender = state.inner.lock().await;
    _ = sender.send(WebViewEvents::OnEcsEvent(event)).await;
    Ok(())
}
