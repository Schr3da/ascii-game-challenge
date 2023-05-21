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
