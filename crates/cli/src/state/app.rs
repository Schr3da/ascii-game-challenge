use std::time::Duration;

use core_logic::prelude::*;

pub struct AppState {
    pub tick_rate: Duration,
    pub ecs: Core,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            ecs: Core::default(),
            tick_rate: Duration::from_millis(30),
        }
    }
}
