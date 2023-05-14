use tokio::sync::mpsc::*;

use crate::prelude::*;
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct Subscription {
    pub sender: Sender<ExternalEvents>,
}

impl Default for Subscription {
    fn default() -> Self {
        let (sender, _) = channel::<ExternalEvents>(1);
        Subscription { sender }
    }
}
