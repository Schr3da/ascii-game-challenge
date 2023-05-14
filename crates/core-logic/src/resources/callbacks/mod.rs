use crate::prelude::*;
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct ExternalSubscriber {
    pub callback: SubscriberCallback,
}

impl Default for ExternalSubscriber {
    fn default() -> Self {
        ExternalSubscriber {
            callback: Box::new(|| {}),
        }
    }
}
