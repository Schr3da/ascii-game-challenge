pub enum SendEvents {
    OnWorldWillUpdate,
}

pub enum SubscriptionEvents {
    OnWorldDidUpdate,
}

pub enum ExternalEvents {
    Send(SendEvents),
    Subscriber(SubscriptionEvents),
}
