use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
