use bevy_ecs::prelude::*;
use async_trait::async_trait;

use crate::prelude::*;

pub struct Core {
    pub world: World,
    pub init_scheduler: InitScheduler,
}

impl Default for Core {
    fn default() -> Self {
        let mut world = World::default();

        let mut init_scheduler = InitScheduler::default();
        init_scheduler.register();

        let mut assets = AssetResources::default();
        assets.load();
        world.insert_resource(assets);

        let subscription = Subscription::default();
        world.insert_resource(subscription);

        Core {
            world,
            init_scheduler,
        }
    }
}

#[async_trait]
impl ExternalEventHandler for Core {
    async fn handle_event(&mut self, event: ExternalEvents) {
        match event {
            ExternalEvents::OnSetSubscriber(s) => {
                let mut next = match self.world.get_resource_mut::<Subscription>() {
                    Some(r) => r,
                    None => return,
                };
                next.sender = s;
            }
            ExternalEvents::OnWorldWillUpdate => {
                self.init_scheduler.run(&mut self.world);
                let next = match self.world.get_resource_mut::<Subscription>() {
                    Some(r) => r,
                    None => return,
                };
                let _ = next.sender.send(ExternalEvents::OnWorldDidUpdate).await;
            }
            ExternalEvents::OnWorldDidUpdate => {}
        }
    }
}
