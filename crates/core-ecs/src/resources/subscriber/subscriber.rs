use bevy_ecs::prelude::*;
use tokio::sync::mpsc::*;

use core_dtos::prelude::*;

#[derive(Resource)]
pub struct Subscriber {
    pub sender: Sender<EcsEvents>,
}
