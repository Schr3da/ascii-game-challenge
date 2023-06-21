use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Tsify)]
pub enum ViewComponentIds {
    Main(MainMenuIds),
    Options(OptionMenuIds),
    Game(GameIds),
    Popup(CommandPopupIds),
}

impl Default for ViewComponentIds {
    fn default() -> Self {
        ViewComponentIds::Main(MainMenuIds::NewGame)
    }
}
