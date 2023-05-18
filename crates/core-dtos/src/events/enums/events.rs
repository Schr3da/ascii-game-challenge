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

pub enum EcsEvents {
    Send(SendEvents),
    Subscriber(SubscriptionEvents),
}
