use bevy_ecs::prelude::*;
use core_shared::prelude::*;
use std::sync::Arc;
use tokio::sync::mpsc::*;
use tokio::sync::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub struct Core {
    pub world: World,
    pub init_scheduler: InitScheduler,
    pub general_scheduler: ApplicationScheduler,
    pub render_scheduler: RenderScheduler,
    pub ui_scheduler: UiScheduler,
    pub tick_scheduler: TickScheduler,
}

impl Default for Core {
    fn default() -> Self {
        let mut world = World::default();

        let mut init_scheduler = InitScheduler::default();
        init_scheduler.setup();

        let mut general_scheduler = ApplicationScheduler::default();
        general_scheduler.setup();

        let mut render_scheduler = RenderScheduler::default();
        render_scheduler.setup();

        let mut ui_scheduler = UiScheduler::default();
        ui_scheduler.setup();

        let mut tick_scheduler = TickScheduler::default();
        tick_scheduler.setup();

        let logger = LoggerResource::default();
        world.insert_resource(logger);

        let mut assets = AssetResources::default();
        assets.load();
        world.insert_resource(assets);

        let game_clock = ClockResource::default();
        world.insert_resource(game_clock);

        let ui_store_resource = UiStoreResource::default();
        world.insert_resource(ui_store_resource);

        let command_state = CommandResource::default();
        world.insert_resource(command_state);

        let camera = CameraResource::default();
        world.insert_resource(camera);

        world.spawn(MenuViews::main_menu());
        world.spawn(MenuViews::settings());
        world.spawn(GameView::new());
        world.spawn(PopupViews::command());
        world.spawn(PopupViews::buildings());
        world.spawn(PopupViews::logger());

        Core {
            world,
            init_scheduler,
            render_scheduler,
            ui_scheduler,
            general_scheduler,
            tick_scheduler,
        }
    }
}

impl ShareableSubscriber<EcsEvents> for Core {
    type Item = Self;

    fn new_shared(subscriber: Sender<EcsEvents>) -> Shared<Self::Item> {
        let mut ecs = Core::default();

        let subsrciber_resource = SubscriberResource::new(subscriber);
        ecs.world.insert_resource(subsrciber_resource);

        Arc::new(RwLock::new(ecs))
    }
}

impl EventHandler for Core {
    fn did_receive(&mut self, event: &SendEvents) {
        match self.world.get_resource_mut::<SubscriberResource>() {
            Some(mut r) => {
                r.previous_event = r.next_event.clone();
                r.next_event = Some(event.clone());
            }
            None => println!("subscriber resource not found"),
        };
    }

    fn handle_event(&mut self, event: SendEvents) {
        self.did_receive(&event);

        match event {
            SendEvents::Ui(e) => self.handle_ui_event(e),
            SendEvents::Renderer(e) => self.handle_renderer_event(e),
            SendEvents::General(e) => self.handle_general_event(e),
            SendEvents::Tick => self.handle_tick_event(),
        }
    }
}

impl Core {
    fn handle_ui_event(&mut self, event: UiEvents) {
        match event {
            UiEvents::OnSelect(_)
            | UiEvents::OnShortcut(_)
            | UiEvents::OnSelectById(_)
            | UiEvents::OnClick(_)
            | UiEvents::OnOpenPopup(_)
            | UiEvents::OnClosePopup
            | UiEvents::OnCloseView => {
                self.ui_scheduler.run(&mut self.world);
            }
        }
    }

    fn handle_renderer_event(&mut self, event: RenderEvents) {
        match event {
            RenderEvents::OnUpdateCamera(_)
            | RenderEvents::OnUpdateSelectedCell(_)
            | RenderEvents::OnWorldWillUpdate => self.render_scheduler.run(&mut self.world),
        }
    }

    fn handle_general_event(&mut self, event: GeneralEvents) {
        match event {
            GeneralEvents::OnApplicationWillInitialise(_, _) => {
                self.init_scheduler.run(&mut self.world)
            }
            GeneralEvents::OnApplicationResize(_, _) => self.general_scheduler.run(&mut self.world),
            GeneralEvents::OnApplicationWillClose => self.general_scheduler.run(&mut self.world),
        }
    }

    fn handle_tick_event(&mut self) {
        match self.world.get_resource::<UiStoreResource>() {
            Some(s) => {
                if s.current_game_status != GameStatus::GameDidStart {
                    return;
                }
            }
            _ => return,
        }

        self.tick_scheduler.run(&mut self.world);
    }
}
