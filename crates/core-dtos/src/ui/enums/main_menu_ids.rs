use crate::prelude::{ToSelectable, ViewComponentIds};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainMenuIds {
    Title,
    MenuList,
    NewGame,
    Options,
    Quit,
}

impl ToString for MainMenuIds {
    fn to_string(&self) -> String {
        match self {
            Self::Title => "Ascii Game Challenge".to_string(),
            Self::MenuList => "Main Menu".to_string(),
            Self::NewGame => "New Game".to_string(),
            Self::Options => "Options".to_string(),
            Self::Quit => "Quit".to_string(),
        }
    }
}

impl ToSelectable for MainMenuIds {
    type Item = ViewComponentIds;

    fn get_selectable_items() -> Vec<ViewComponentIds> {
        vec![
            ViewComponentIds::Main(MainMenuIds::NewGame),
            ViewComponentIds::Main(MainMenuIds::Options),
            ViewComponentIds::Main(MainMenuIds::Quit),
        ]
    }
}
