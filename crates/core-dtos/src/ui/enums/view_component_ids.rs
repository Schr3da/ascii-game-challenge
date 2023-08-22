use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash, Tsify)]
pub enum ViewComponentIds {
    Main(MainMenuIds),
    Options(OptionMenuIds),
    Game(GameIds),
    Popup(PopupIds),
}

impl Default for ViewComponentIds {
    fn default() -> Self {
        ViewComponentIds::Main(MainMenuIds::NewGame)
    }
}

impl ToRoute for ViewComponentIds {
    fn to_view_route(&self) -> Option<UiViewIds> {
        match self {
            Self::Main(MainMenuIds::NewGame) => Some(UiViewIds::Game),
            Self::Main(MainMenuIds::Options) => Some(UiViewIds::Options),
            Self::Main(MainMenuIds::Quit) => Some(UiViewIds::Quit),
            Self::Options(n) => n.to_view_route(),
            _ => None,
        }
    }

    fn to_popup_route(&self) -> Option<UiPopupViewIds> {
        match self {
            Self::Popup(PopupIds::Build(_)) => Some(UiPopupViewIds::Buildings),
            Self::Popup(PopupIds::Logger(_)) => Some(UiPopupViewIds::Logger),
            _ => None,
        }
    } 
}