use crate::export::prelude::*;
use core_dtos::prelude::*;

#[tauri::command]
pub async fn webview_input_event(
    event: InputEvents,
    state: tauri::State<'_, JsSignal>,
) -> Result<(), ()> {
    let sender = state.inner.lock().await;
    _ = sender.send(WebViewEvents::OnInputEvent(event)).await;
    Ok(())
}
