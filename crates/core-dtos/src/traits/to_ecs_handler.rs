use crate::prelude::*;

pub trait EventHandler {
    fn did_receive(&mut self, event: &SendEvents);

    fn handle_event(&mut self, event: SendEvents);
}
