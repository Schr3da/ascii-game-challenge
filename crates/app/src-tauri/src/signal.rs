use tauri::async_runtime::Mutex;
use tokio::sync::mpsc::*;

pub struct JsSignal {
    pub inner: Mutex<Sender<String>>,
}

impl JsSignal {
    pub fn new() -> (Self, Receiver<String>) {
        let (sender, receiver) = channel::<String>(1);

        let signal = JsSignal {
            inner: Mutex::new(sender),
        };

        (signal, receiver)
    }
}
