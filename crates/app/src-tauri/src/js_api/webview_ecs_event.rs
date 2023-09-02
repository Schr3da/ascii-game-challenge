use crate::export::prelude::*;
use core_dtos::prelude::*;

#[tauri::command]
pub async fn webview_ecs_event(
    event: SendEvents,
    state: tauri::State<'_, TauriBridge>,
) -> Result<(), ()> {
    let sender = state.inner.lock().await;
    _ = sender.send(TauriWebViewEvents::OnEcsEvent(event)).await;
    Ok(())
}
