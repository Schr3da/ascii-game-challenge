use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct Logger {
    data: Vec<String>,
}

impl From<&Logger> for ViewDataTypes {
    fn from(value: &Logger) -> Self {
        ViewDataTypes::Logger(LoggerState {
            current_logs: value.data.clone(),
        })
    }
}

impl Logger {
    pub fn log(&mut self, next: String) {
        self.data.push(next);
    }
}
