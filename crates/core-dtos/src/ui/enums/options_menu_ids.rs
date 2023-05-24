use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionMenu {
    Title,
    OptionList,
    Back,
}

impl ToString for OptionMenu {
    fn to_string(&self) -> String {
        match self {
            Self::Title => "Options".to_string(),
            Self::OptionList => "Game Options".to_string(),
            Self::Back => "Back".to_string(),
        }
    }
}

impl ToSelectable for OptionMenu {
    type Item = ViewComponentIds;

    fn get_selectable_items() -> Vec<ViewComponentIds> {
        vec![]
    }
}
