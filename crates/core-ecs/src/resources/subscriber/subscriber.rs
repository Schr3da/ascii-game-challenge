use bevy_ecs::prelude::*;
use tokio::sync::mpsc::*;

use crate::prelude::*;

#[derive(Resource)]
pub struct Subscriber {
    pub sender: Sender<ExternalEvents>,
}
