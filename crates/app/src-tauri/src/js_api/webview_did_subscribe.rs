use crate::export::prelude::*;

#[tauri::command]
pub async fn webview_did_subscribe(
    has_subscribed: bool,
    state: tauri::State<'_, JsSignal>,
) -> Result<(), ()> {
    if !has_subscribed {
        return Ok(());
    }

    let sender = state.inner.lock().await;
    _ = sender.send(WebViewEvents::OnDidSubscribe).await;
    Ok(())
}
