use std::sync::{Arc, Mutex};

pub type Shared<T> = Arc<Mutex<T>>; 

pub trait Shareable {
    type Item: Default;

    fn new_shared() -> Shared<Self::Item>;
}
