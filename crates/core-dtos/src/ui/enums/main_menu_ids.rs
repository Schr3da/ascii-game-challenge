use crate::prelude::{ToSelectable, ViewComponentIds};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainMenu {
    Title,
    MenuList,
    NewGame,
    Options,
    Quit,
}

impl ToString for MainMenu {
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

impl ToSelectable for MainMenu {
    type Item = ViewComponentIds;

    fn get_selectable_items() -> Vec<ViewComponentIds> {
        vec![
            ViewComponentIds::Main(MainMenu::NewGame),
            ViewComponentIds::Main(MainMenu::Options),
            ViewComponentIds::Main(MainMenu::Quit),
        ]
    }
}
