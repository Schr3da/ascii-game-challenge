use crate::prelude::*;
use async_trait::async_trait;

#[async_trait]
pub trait ExternalEventHandler {
    async fn handle_event(&mut self, event: ExternalEvents);
}
