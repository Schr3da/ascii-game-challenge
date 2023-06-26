use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Clone, Deserialize, Serialize, Tsify)]
pub enum QuickActionEvents {
    New,
    Execute,
    Cancel,
}

#[derive(Debug, Clone, Eq, PartialEq, Tsify, Serialize, Deserialize)]
pub enum QuickNavigationSubscription {
    OnQuickActionDidUpdate(String),
}
