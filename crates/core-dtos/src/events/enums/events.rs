use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Clone, Tsify, Serialize, Deserialize)]
pub enum SendEvents {
    General(GeneralEvents),
    Ui(UiEvents),
    Commands(CommandInputEvents),
    QuickAction(QuickActionEvents),
    Renderer(RenderEvents),
}

#[derive(Debug, Clone, Eq, PartialEq, Tsify, Serialize, Deserialize)]
pub enum SubscriptionEvents {
    General(GeneralSubscription),
    Ui(UiSubscription),
    Renderer(RenderSubscription),
    Command(CommandSubscription),
}

#[derive(Clone, Tsify)]
pub enum EcsEvents {
    Send(SendEvents),
    Subscriber(SubscriptionEvents),
}
