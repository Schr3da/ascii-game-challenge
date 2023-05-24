use crate::prelude::*;

#[derive(Clone)]
pub enum SendEvents {
    General(GeneralEvents),
    Ui(UiEvents),
    Renderer(RenderEvents),
}

#[derive(Clone, Eq, PartialEq)]
pub enum SubscriptionEvents {
    General(GeneralSubscription),
    Ui(UiSubscription),
    Renderer(RenderSubscription),
}

#[derive(Clone)]
pub enum EcsEvents {
    Send(SendEvents),
    Subscriber(SubscriptionEvents),
}
