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

        Core {
            world,
            init_scheduler,
        }
    }
}