use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash, Tsify)]
pub enum ViewComponentIds {
    Main(MainMenuIds),
    Options(OptionMenuIds),
    Game(GameIds),
    CommandPopup(CommandIds),
}

impl Default for ViewComponentIds {
    fn default() -> Self {
        ViewComponentIds::Main(MainMenuIds::NewGame)
    }
}

impl ToRoute for ViewComponentIds{
    fn to_view_route(&self) -> Option<UiViewIds> {
        match self {
            Self::Main(MainMenuIds::NewGame) => Some(UiViewIds::Game), 
            Self::Main(MainMenuIds::Options) => Some(UiViewIds::Options), 
            _ => None,
        }
    }

    fn to_popup_route(&self) -> Option<UiPopupViewIds> {
        match self {
            Self::CommandPopup(CommandIds::Build(_)) => Some(UiPopupViewIds::Buildings),
            _ => None,
        }
        
    }
}
