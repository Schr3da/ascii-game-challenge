use bevy_ecs::prelude::*;
use core_shared::prelude::*;
use std::sync::Arc;
use tokio::sync::mpsc::*;
use tokio::sync::*;

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

        Core {
            world,
            init_scheduler,
        }
    }
}

impl ShareableSubscriber<ExternalEvents> for Core {
    type Item = Self;

    fn new_shared(subscriber: Sender<ExternalEvents>) -> Shared<Self::Item> {
        let mut ecs = Core::default();

        ecs.world.insert_resource(Subscriber { sender: subscriber });

        Arc::new(RwLock::new(ecs))
    }
}

impl ExternalEventHandler for Core {
    fn handle_event(&mut self, event: SendEvents) {
        match event {
            SendEvents::OnWorldWillUpdate => {
                self.init_scheduler.run(&mut self.world);
            }
        }
    }
}
