#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InGameIds {
    None,
}

impl ToString for InGameIds {
    fn to_string(&self) -> String {
        match self {
            Self::None => "".to_string(),
        }
    }
}
