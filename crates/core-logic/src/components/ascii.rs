use serde::*;

#[derive(Clone, Serialize, Deserialize)]
pub enum Ascii {
    Space,
    Plus,
}

impl Default for Ascii {
    fn default() -> Self {
        Ascii::Space
    }
}

impl ToString for Ascii {
    fn to_string(&self) -> String {
        match self {
            Ascii::Space => " ".to_string(),
            Ascii::Plus => "+".to_string(),
        }
    }
}
