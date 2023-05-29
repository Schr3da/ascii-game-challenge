use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ViewComponentIds {
    Main(MainMenuIds),
    Options(OptionMenuIds),
    Game(GameIds),
}

impl Default for ViewComponentIds {
    fn default() -> Self {
        ViewComponentIds::Main(MainMenuIds::NewGame)
    }
}
