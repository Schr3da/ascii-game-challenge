use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone, Default, Serialize, Deserialize, Tsify)]
pub enum ViewDataTypes {
    #[default]
    NoViewData,
    GameHeader(GameViewHeaderState),
    Popup(PopupState),
}

impl From<&ViewDataTypes> for HashMap<ViewComponentIds, String> {
    fn from(value: &ViewDataTypes) -> Self {
        match value {
            ViewDataTypes::GameHeader(s) => s.into(),
            ViewDataTypes::NoViewData | ViewDataTypes::Popup(_) => HashMap::new(),
        }
    }
}
