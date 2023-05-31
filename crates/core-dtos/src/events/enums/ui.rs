use serde::{Serialize, Deserialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Clone, Tsify, Serialize, Deserialize)]
pub enum UiEvents {
    OnSelect(SelectionDirections),
    OnClick(ViewComponentIds),
}

#[derive(Clone, Eq, PartialEq, Tsify, Serialize, Deserialize)]
pub enum UiSubscription {
    UnknownUiSubscription
}
