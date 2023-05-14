use bevy_ecs::prelude::*;

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

        let subscriber = ExternalSubscriber::default();
        world.insert_resource(subscriber);

        Core {
            world,
            init_scheduler,
        }
    }
}

impl ExternalEventHandler for Core {
    fn handle_event(&mut self, event: ExternalEvents) {
        match event {
            ExternalEvents::OnSetSubscriber(cb) => self.subscribe(cb),
        }
    }
}

impl Core {
    fn subscribe(&mut self, cb: SubscriberCallback) {
        let mut next = match self.world.get_resource_mut::<ExternalSubscriber>() {
            Some(r) => r,
            None => return,
        };
        next.callback = cb;
    }
}
