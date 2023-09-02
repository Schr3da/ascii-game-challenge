use crate::export::prelude::*;
use core_dtos::prelude::*;

#[tauri::command]
pub async fn webview_input_event(
    event: InputEvents,
    state: tauri::State<'_, TauriBridge>,
) -> Result<(), ()> {
    let sender = state.inner.lock().await;
    _ = sender.send(TauriWebViewEvents::OnInputEvent(event)).await;
    Ok(())
}
