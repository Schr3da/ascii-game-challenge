use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Tsify)]
pub enum UiViewIds {
    Main,
    Game,
    Options,
    Popup(UiPopupViewIds),
}

impl Default for UiViewIds {
    fn default() -> Self {
        UiViewIds::Main
    }
}
