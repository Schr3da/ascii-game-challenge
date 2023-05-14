use std::time::Duration;
use tokio::sync::mpsc::*;

use core_logic::prelude::*;

pub struct AppState {
    pub tick_rate: Duration,
    pub ecs: Core,
    pub subscription_sender: Sender<ExternalEvents>,
    pub subscription_receiver: Receiver<ExternalEvents>,
}

impl Default for AppState {
    fn default() -> Self {
        let (sender, receiver) = channel::<ExternalEvents>(2);

        AppState {
            ecs: Core::default(),
            tick_rate: Duration::from_millis(30),
            subscription_sender: sender,
            subscription_receiver: receiver,
        }
    }
}

impl AppState {
    pub fn subscribe(&mut self) {
        let _ = self.ecs.handle_event(ExternalEvents::OnSetSubscriber(
            self.subscription_sender.clone(),
        ));
    }

    pub async fn process(&mut self, event: ExternalEvents) {
        self.ecs.handle_event(event).await;
    }
}
