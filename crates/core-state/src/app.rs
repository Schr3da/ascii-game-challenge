use std::sync::Arc;

use core_ecs::prelude::*;
use core_shared::prelude::*;
use tokio::sync::{mpsc::*, RwLock};
use tokio::task;

pub struct AppState {
    pub ecs: Shared<Core>,
    pub task: Option<task::JoinHandle<()>>,
    pub core_event_sender: Sender<ExternalEvents>,
    pub core_event_receiver: Shared<Receiver<ExternalEvents>>,
    pub subscription_sender: Sender<SubscriptionEvents>,
    pub subscription_receiver: Receiver<SubscriptionEvents>,
}

impl Default for AppState {
    fn default() -> Self {
        let (core_event_sender, core_event_receiver) = channel::<ExternalEvents>(2);
        let (subscription_sender, subscription_receiver) = channel::<SubscriptionEvents>(2);

        AppState {
            ecs: Core::new_shared(core_event_sender.clone()),
            task: None,
            core_event_sender,
            core_event_receiver: Arc::new(RwLock::new(core_event_receiver)),
            subscription_sender,
            subscription_receiver,
        }
    }
}

impl AppState {
    pub fn subscribe(&mut self) {
        self.unsubscribe();

        let scoped_ecs = self.ecs.clone();

        let scoped_core_receiver = self.core_event_receiver.clone();

        let scoped_subcription_sender = self.subscription_sender.clone();

        let task = tokio::spawn(async move {
            let mut core_receiver = scoped_core_receiver.write().await;
            let mut ecs = scoped_ecs.write().await;

            loop {
                match core_receiver.recv().await {
                    Some(ExternalEvents::Send(e)) => ecs.handle_event(e),
                    Some(ExternalEvents::Subscriber(e)) => {
                        let _ = scoped_subcription_sender.send(e).await;
                    }
                    _ => continue,
                };
            }
        });

        self.task = Some(task);
    }

    pub fn unsubscribe(&mut self) {
        match &self.task {
            Some(t) => t.abort(),
            _ => {}
        };

        self.task = None;
    }

    pub async fn send(&mut self, event: SendEvents) {
        let _ = self
            .core_event_sender
            .send(ExternalEvents::Send(event))
            .await;
    }
}

impl Drop for AppState {
    fn drop(&mut self) {
        self.unsubscribe()
    }
}
