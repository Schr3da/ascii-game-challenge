use serde::{Serialize, Deserialize};
use tsify::Tsify;

#[derive(Clone, Tsify, Serialize, Deserialize)]
pub enum GeneralEvents {
    OnApplicationResize(u16, u16),
    OnApplicationWillInitialise(u16, u16),
    OnApplicationWillClose,
}

#[derive(Clone, Eq, PartialEq, Tsify, Serialize, Deserialize)]
pub enum GeneralSubscription {
    OnApplicationDidInitialise,
    OnApplicationDidClose,
}
