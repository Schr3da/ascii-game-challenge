use tokio::sync::mpsc::*;

pub enum ExternalEvents {
    OnSetSubscriber(Sender<ExternalEvents>),
    OnWorldWillUpdate,
    OnWorldDidUpdate,
}
