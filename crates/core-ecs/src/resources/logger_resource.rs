use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct LoggerResource {
    data: Vec<String>,
}

impl From<&LoggerResource> for ViewDataTypes {
    fn from(value: &LoggerResource) -> Self {
        ViewDataTypes::Logger(LoggerState {
            current_logs: value.data.clone(),
        })
    }
}

impl LoggerResource {
    pub fn log(&mut self, next: String) {
        self.data.push(next);
    }
}
