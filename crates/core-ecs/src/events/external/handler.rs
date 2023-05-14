use crate::prelude::*;

pub trait ExternalEventHandler {
    fn handle_event(&mut self, event: ExternalSendEvents);
}

impl ExternalEventHandler for Core {
    fn handle_event(&mut self, event: ExternalSendEvents) {
        match event {
            ExternalSendEvents::OnWorldWillUpdate => {
                self.init_scheduler.run(&mut self.world);
            }
        }
    }
}
