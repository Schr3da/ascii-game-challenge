use bevy_ecs::prelude::*;
use core_shared::prelude::*;
use std::sync::Arc;
use tokio::sync::mpsc::*;
use tokio::sync::*;

use crate::prelude::*;

pub struct Core {
    pub world: World,
    pub init_scheduler: InitScheduler,
    pub render_scheduler: RenderScheduler,
    pub ui_scheduler: UiScheduler,
    pub general_scheduler: GeneralScheduler,
}

impl Default for Core {
    fn default() -> Self {
        let mut world = World::default();

        let mut init_scheduler = InitScheduler::default();
        init_scheduler.register();

        let mut render_scheduler = RenderScheduler::default();
        render_scheduler.register();

        let mut ui_scheduler = UiScheduler::default();
        ui_scheduler.register();

        let mut general_scheduler = GeneralScheduler::default();
        general_scheduler.register();

        let mut assets = AssetResources::default();
        assets.load();
        world.insert_resource(assets);

        Core {
            world,
            init_scheduler,
            render_scheduler,
            ui_scheduler,
            general_scheduler,
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
            SendEvents::General(e) => self.handle_general_event(e),
        }
    }

    fn handle_ui_event(&mut self, event: UiEvents) {
        match event {
            UiEvents::OnClick => {
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

    fn handle_general_event(&mut self, event: GeneralEvents) {
        match event {
            GeneralEvents::OnApplicationWillInitialise => self.init_scheduler.run(&mut self.world),
            GeneralEvents::OnApplicationWillClose => self.general_scheduler.run(&mut self.world),
        }
    }
}
