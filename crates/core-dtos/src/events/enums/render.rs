use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub enum RenderEvents {
    OnWorldWillUpdate,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum RenderSubscription {
    OnWorldDidUpdate(Option<UiView>),
}
