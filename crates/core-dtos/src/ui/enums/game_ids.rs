use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameIds {
    Title,
    None,
}

impl ToString for GameIds {
    fn to_string(&self) -> String {
        match self {
            Self::Title => "Game View".to_string(),
            Self::None => "".to_string(),
        }
    }
}

impl ToSelectable for GameIds {
    type Item = ViewComponentIds;

    fn get_selectable_items() -> Vec<ViewComponentIds> {
        vec![]
    }
}
