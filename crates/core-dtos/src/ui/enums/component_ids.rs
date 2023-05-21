use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewComponentIds {
    Main(MainMenu),
    Options(OptionMenu),
    Game(InGameIds),
}

impl Default for ViewComponentIds {
    fn default() -> Self {
        ViewComponentIds::Main(MainMenu::NewGame)
    }
}