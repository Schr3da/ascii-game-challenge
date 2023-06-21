use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Clone, Tsify, Serialize, Deserialize)]
pub enum UiEvents {
    OnSelect(SelectionDirections),
    OnSelectById(ViewComponentIds),
    OnClick(ViewComponentIds),
    OnCloseView,
}

#[derive(Debug, Clone, Eq, PartialEq, Tsify, Serialize, Deserialize)]
pub enum UiSubscription {
    UnknownUiSubscription,
}
