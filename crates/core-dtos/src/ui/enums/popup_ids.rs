use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Tsify)]
pub enum UiPopupViewIds {
    Command,
    Buildings,
}

impl Default for UiPopupViewIds {
    fn default() -> Self {
        UiPopupViewIds::Command
    }
}
