use crate::prelude::*;

pub enum SendEvents {
    General(GeneralEvents),
    Ui(UiEvents),
    Renderer(RenderEvents),
}

pub enum SubscriptionEvents {
    General(GeneralSubscription),
    Ui(UiSubscription),
    Renderer(RenderSubscription),
}

pub enum ExternalEvents {
    Send(SendEvents),
    Subscriber(SubscriptionEvents),
}
