use tauri::async_runtime::Mutex;
use tokio::sync::mpsc::*;

use crate::export::prelude::*;

pub struct JsSignal {
    pub inner: Mutex<Sender<WebViewEvents>>,
}

impl JsSignal {
    pub fn new() -> (Self, Receiver<WebViewEvents>) {
        let (sender, receiver) = channel::<WebViewEvents>(1);

        let signal = JsSignal {
            inner: Mutex::new(sender),
        };

        (signal, receiver)
    }
}
