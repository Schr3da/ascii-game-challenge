use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Tsify)]
pub enum UiPopupViewIds {
  Input,
}

impl Default for UiPopupViewIds {
    fn default() -> Self {
      UiPopupViewIds::Input
    }
}
