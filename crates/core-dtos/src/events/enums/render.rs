use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Clone, Tsify, Serialize, Deserialize)]
pub enum RenderEvents {
    OnWorldWillUpdate,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Tsify)]
pub enum RenderSubscription {
    OnWorldDidUpdate(Option<UiView>, Option<UiView>),
}
