use tsify::Tsify;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Tsify, Serialize, Deserialize)]
pub enum RenderEvents {
    OnWorldWillUpdate,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Tsify)]
pub enum RenderSubscription {
    OnWorldDidUpdate(Option<UiView>),
}
