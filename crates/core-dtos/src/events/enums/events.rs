use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum SendEvents {
    General(GeneralEvents),
    Ui(UiEvents),
    Renderer(RenderEvents),
    Tick,
}

#[derive(Debug, Clone, Eq, PartialEq, Tsify, Serialize, Deserialize)]
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
