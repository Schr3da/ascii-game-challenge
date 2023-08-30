use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use tokio::sync::mpsc::Sender;

#[derive(Resource)]
pub struct SubscriberResource {
    pub previous_event: Option<SendEvents>,
    pub next_event: Option<SendEvents>,
    pub sender: Sender<EcsEvents>,
}

impl Constructable for SubscriberResource {
    type Args = Sender<EcsEvents>;

    fn new(sender: Self::Args) -> Self {
        SubscriberResource {
            previous_event: None,
            next_event: None,
            sender,
        }
    }
}
