use core_ecs::prelude::*;

pub struct AppState {
    pub ecs: Core,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            ecs: Core::default(),
        }
    }
}

impl AppState {
    pub fn send(&mut self, event: ExternalSendEvents) {
        self.ecs.handle_event(event);
    }
}
