use crate::prelude::*;

pub trait ExternalEventHandler {
    fn handle_event(&mut self, event: SendEvents);
}
