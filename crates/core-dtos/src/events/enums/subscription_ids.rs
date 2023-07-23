use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Clone, Tsify, Serialize, Deserialize)]
pub enum EcsSubscriptionIds {
    GeneralSubscription,
    ViewSubscription,
    PopupSubscription,
    GameStatusSubscription,
}

impl ToString for EcsSubscriptionIds {
    fn to_string(&self) -> String {
        match self {
            Self::GeneralSubscription => "GeneralSubscription".to_string(),
            Self::ViewSubscription => "ViewSubscription".to_string(),
            Self::PopupSubscription => "PopupSubscription".to_string(),
            Self::GameStatusSubscription => "GameStatusSubscription".to_string(),
        }
    }
}
