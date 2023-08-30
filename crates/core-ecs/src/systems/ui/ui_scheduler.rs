use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

use crate::prelude::*;

pub struct UiScheduler {
    pub schedule: Schedule,
}

impl Default for UiScheduler {
    fn default() -> Self {
        let schedule = Schedule::default();

        UiScheduler { schedule }
    }
}

impl ToScheduler for UiScheduler {
    fn setup(&mut self) {
        self.schedule.add_systems((
            UiSystems::on_open_popup,
            UiSystems::on_close_popup,
            UiSystems::on_select,
            UiSystems::on_select_by_id,
            UiSystems::on_click,
            UiSystems::on_shortcut,
            UiSystems::on_close_view,
            UiSystems::on_ui_did_update.after(UiSystems::on_click),
        ));
    }

    fn run(&mut self, world: &mut World) {
        self.schedule.run(world);
    }
}
