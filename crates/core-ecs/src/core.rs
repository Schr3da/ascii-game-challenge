use bevy_ecs::prelude::*;
use core_shared::prelude::*;
use std::sync::Arc;
use tokio::sync::mpsc::*;
use tokio::sync::*;

use crate::prelude::*;

pub struct Core {
    pub world: World,
    pub render_scheduler: RenderScheduler,
    pub ui_scheduler: RenderScheduler,
}

impl Default for Core {
    fn default() -> Self {
        let mut world = World::default();

        let mut render_scheduler = RenderScheduler::default();
        render_scheduler.register();

        let mut ui_scheduler = RenderScheduler::default();
        ui_scheduler.register();

        let mut assets = AssetResources::default();
        assets.load();
        world.insert_resource(assets);

        Core {
            world,
            render_scheduler,
            ui_scheduler,
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
            SendEvents::Ui(e) => self.handle_ui_event(e),
            SendEvents::Renderer(e) => self.handle_game_event(e),
        }
    }

    fn handle_ui_event(&mut self, event: UiEvents) {
        match event {
            UiEvents::OnWorldWillUpdate => {
                self.render_scheduler.run(&mut self.world);
            }
        }
    }

    fn handle_game_event(&mut self, event: RenderEvents) {
        match event {
            RenderEvents::OnWorldWillUpdate => {
                self.render_scheduler.run(&mut self.world);
            }
        }
    }
}
