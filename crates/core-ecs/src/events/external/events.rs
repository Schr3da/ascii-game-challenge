
pub enum RenderEvents {
    OnWorldWillUpdate,
}

pub enum UiEvents {
    OnWorldWillUpdate,
}

pub enum SendEvents {
    Ui(UiEvents),
    Renderer(RenderEvents),
}

pub enum UiSubscription {

}

pub enum RenderSubscription {
    OnWorldDidUpdate,
}

pub enum SubscriptionEvents {
    Ui(UiSubscription),
    Game(RenderSubscription),
}

pub enum ExternalEvents {
    Send(SendEvents),
    Subscriber(SubscriptionEvents),
}
