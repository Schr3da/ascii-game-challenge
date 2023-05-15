use std::sync::Arc;
use tokio::sync::mpsc::*;
use tokio::sync::*;

pub type Shared<T> = Arc<RwLock<T>>;

pub trait ShareableSubscriber<T> {
    type Item: Default;
    fn new_shared(subscriber: Sender<T>) -> Shared<Self::Item>;
}

pub trait Shareable {
    type Item: Default;
    fn new_shared() -> Shared<Self::Item>;
}
