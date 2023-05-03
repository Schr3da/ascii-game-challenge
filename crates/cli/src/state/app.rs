use std::time::Duration;

pub struct AppState {
    pub tick_rate: Duration,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            tick_rate: Duration::from_millis(30),
        }
    }
}
