use tauri::async_runtime::Mutex;
use tokio::sync::mpsc::*;

use crate::export::prelude::*;

pub struct TauriBridge {
    pub inner: Mutex<Sender<TauriWebViewEvents>>,
}

impl TauriBridge {
    pub fn new() -> (Self, Receiver<TauriWebViewEvents>) {
        let (sender, receiver) = channel::<TauriWebViewEvents>(1);

        let signal = TauriBridge {
            inner: Mutex::new(sender),
        };

        (signal, receiver)
    }
}
