use crate::export::prelude::*;

#[tauri::command]
pub async fn webview_did_mount(
    is_mounted: bool,
    state: tauri::State<'_, JsSignal>,
) -> Result<(), ()> {
    if !is_mounted {
        return Ok(());
    }

    let sender = state.inner.lock().await;
    _ = sender.send(WebViewEvents::OnDidMount).await;
    Ok(())
}
