use std::sync::Arc;

use tokio::sync::{mpsc::*, RwLock};
use tokio::task;

use core_dtos::prelude::*;
use core_ecs::prelude::*;
use core_shared::prelude::*;

use crate::prelude::*;

pub struct AppState {
    pub ecs: Shared<Core>,
    pub ecs_task: Option<task::JoinHandle<()>>,
    pub ecs_event_sender: Sender<EcsEvents>,
    pub ecs_event_receiver: Shared<Receiver<EcsEvents>>,
    pub ecs_subscription_sender: Sender<SubscriptionEvents>,
    pub ecs_subscription_receiver: Receiver<SubscriptionEvents>,
    pub ecs_current_view_state: Option<UiViewState>,
    pub ecs_current_popup_state: Option<UiViewState>,
}

impl Default for AppState {
    fn default() -> Self {
        let (core_event_sender, core_event_receiver) = channel::<EcsEvents>(10);
        let (subscription_sender, subscription_receiver) = channel::<SubscriptionEvents>(10);

        AppState {
            ecs_task: None,
            ecs: Core::new_shared(core_event_sender.clone()),
            ecs_event_sender: core_event_sender,
            ecs_event_receiver: Arc::new(RwLock::new(core_event_receiver)),
            ecs_subscription_sender: subscription_sender,
            ecs_subscription_receiver: subscription_receiver,
            ecs_current_view_state: None,
            ecs_current_popup_state: None,
        }
    }
}

impl Drop for AppState {
    fn drop(&mut self) {
        self.unsubscribe();
    }
}
