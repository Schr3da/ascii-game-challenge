use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

pub static TICK: i32 = 1;

static INTERVAL: f32 = 15.0;

static HOURS_PER_DAY: f32 = 24.0;

static MINUTES_PER_HOUR: f32 = 60.0;

static MINUTES_PER_DAY: f32 = 24.0 * MINUTES_PER_HOUR;

#[derive(Resource)]
pub struct ClockResource {
    pub days: i32,
    pub hours: i32,
    pub minutes: i32,
    pub total_minutes: i32,
    pub ticks: i32,
}

impl Default for ClockResource {
    fn default() -> Self {
        ClockResource {
            days: DEFAULT_DAYS,
            hours: DEFAULT_HOURS,
            minutes: 0,
            total_minutes: 0,
            ticks: 0,
        }
    }
}

impl ClockResource {
    pub fn update(&mut self) {
        let time = self.total_minutes as f32 + INTERVAL;

        let absolute_days = (time / MINUTES_PER_DAY).floor();
        let days_in_hours = (absolute_days * MINUTES_PER_DAY).floor();

        let mut hours = ((time - days_in_hours) / MINUTES_PER_HOUR).floor();
        if hours % HOURS_PER_DAY == 0.0 {
            hours = 0.0;
        }

        let mut minutes = time % MINUTES_PER_HOUR;
        if minutes % MINUTES_PER_HOUR == 0.0 {
            minutes = 0.0;
        }

        self.ticks += TICK;
        self.total_minutes = time as i32;
        self.days = absolute_days as i32 + 1;
        self.hours = hours as i32;
        self.minutes = minutes as i32;
    }
}
