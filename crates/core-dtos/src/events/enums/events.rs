use serde::{Serialize, Deserialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Clone, Tsify)]
pub enum SendEvents {
    General(GeneralEvents),
    Ui(UiEvents),
    Renderer(RenderEvents),
}

#[derive(Clone, Eq, PartialEq, Tsify, Serialize, Deserialize)]
pub enum SubscriptionEvents {
    General(GeneralSubscription),
    Ui(UiSubscription),
    Renderer(RenderSubscription),
}

#[derive(Clone, Tsify)]
pub enum EcsEvents {
    Send(SendEvents),
    Subscriber(SubscriptionEvents),
}
