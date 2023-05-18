use bevy_ecs::prelude::*;
use tokio::sync::mpsc::*;

use core_dtos::prelude::*;

#[derive(Resource)]
pub struct Subscriber {
    pub previous_event: Option<SendEvents>,
    pub next_event: Option<SendEvents>,
    pub sender: Sender<EcsEvents>,
}

impl Constructable for Subscriber {
    type Args = Sender<EcsEvents>;

    fn new(sender: Self::Args) -> Self {
        Subscriber {
            previous_event: None,
            next_event: None,
            sender,
        }
    }
}
