use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum OptionMenuIds {
    Title,
    OptionList,
    Back,
}

impl ToString for OptionMenuIds {
    fn to_string(&self) -> String {
        match self {
            Self::Title => "Options".to_string(),
            Self::OptionList => "Game Options".to_string(),
            Self::Back => "Back".to_string(),
        }
    }
}

impl ToSelectable for OptionMenuIds {
    type Item = ViewComponentIds;

    fn get_selectable_items() -> Vec<ViewComponentIds> {
        vec![]
    }
}
