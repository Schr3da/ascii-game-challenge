use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
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
