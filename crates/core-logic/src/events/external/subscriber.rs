pub type SubscriberCallback = Box<dyn Fn() + Sync + Send + 'static>;
