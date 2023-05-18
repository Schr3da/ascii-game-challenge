use async_trait::async_trait;

use core_ecs::prelude::*;

use crate::prelude::*;

#[async_trait]
pub trait EcsState {
    fn subscribe(&mut self);

    fn unsubscribe(&mut self);

    async fn send(&mut self, event: SendEvents);
}

#[async_trait]
impl EcsState for AppState {
    fn subscribe(&mut self) {
        <Self as EcsState>::unsubscribe(self);

        let scoped_ecs = self.ecs.clone();

        let scoped_core_receiver = self.ecs_event_receiver.clone();

        let scoped_subcription_sender = self.ecs_subscription_sender.clone();

        let task = tokio::task::spawn(async move {
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

        self.ecs_task = Some(task);
    }

    fn unsubscribe(&mut self) {
        match &self.ecs_task {
            Some(t) => t.abort(),
            _ => {}
        };

        self.ecs_task = None;
    }

    async fn send(&mut self, event: SendEvents) {
        let _ = self
            .ecs_event_sender
            .send(ExternalEvents::Send(event))
            .await;
    }
}
