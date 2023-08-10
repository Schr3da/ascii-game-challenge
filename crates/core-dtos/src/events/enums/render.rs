use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum RenderEvents {
    OnUpdateCamera(CameraNavigation),
    OnUpdateSelectedCell(SelectedCellNavigation),
    OnWorldWillUpdate,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Tsify)]
pub enum RenderSubscription {
    OnWorldDidUpdate(Option<UiView>, Option<UiView>, GameMeta),
}
