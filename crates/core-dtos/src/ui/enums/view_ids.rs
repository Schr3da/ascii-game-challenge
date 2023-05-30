use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Tsify)]
pub enum UiViewIds {
    Main,
    Game,
    Options,
}

impl Default for UiViewIds {
    fn default() -> Self {
        UiViewIds::Main
    }
}
