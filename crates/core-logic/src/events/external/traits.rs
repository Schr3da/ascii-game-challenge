use crate::prelude::*;

pub enum ExternalEvents {
    OnSetSubscriber(SubscriberCallback),
}

pub trait ExternalEventHandler {
    fn handle_event(&mut self, event: ExternalEvents);
}
