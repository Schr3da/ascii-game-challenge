use crate::export::prelude::*;

#[tauri::command]
pub async fn did_webview_mount(
    is_mounted: bool,
    state: tauri::State<'_, JsSignal>,
) -> Result<(), ()> {
    if !is_mounted {
        return Ok(());
    }

    let sender = state.inner.lock().await;
    _ = sender.send("".to_string()).await;
    Ok(())
}
