use std::sync::Arc;

use core_ecs::prelude::*;
use core_shared::prelude::*;
use tokio::sync::{mpsc::*, RwLock};
use tokio::task;

use crate::prelude::*;

pub struct AppState {
    pub ecs: Shared<Core>,
    pub ecs_task: Option<task::JoinHandle<()>>,
    pub ecs_event_sender: Sender<ExternalEvents>,
    pub ecs_event_receiver: Shared<Receiver<ExternalEvents>>,
    pub ecs_subscription_sender: Sender<SubscriptionEvents>,
    pub ecs_subscription_receiver: Receiver<SubscriptionEvents>,
}

impl Default for AppState {
    fn default() -> Self {
        let (core_event_sender, core_event_receiver) = channel::<ExternalEvents>(2);
        let (subscription_sender, subscription_receiver) = channel::<SubscriptionEvents>(2);

        AppState {
            ecs: Core::new_shared(core_event_sender.clone()),
            ecs_task: None,
            ecs_event_sender: core_event_sender,
            ecs_event_receiver: Arc::new(RwLock::new(core_event_receiver)),
            ecs_subscription_sender: subscription_sender,
            ecs_subscription_receiver: subscription_receiver,
        }
    }
}

impl Drop for AppState {
    fn drop(&mut self) {
        <Self as EcsState>::unsubscribe(self);
    }
}
