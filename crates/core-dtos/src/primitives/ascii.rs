use std::fmt;

use tsify::Tsify;

use serde::{de::Visitor, *};

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash, Tsify)]
#[serde(rename_all = "camelCase")]
pub enum Ascii {
    Space,
    Plus,
    Tilde,
    DoubleTilde,
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
            Ascii::Tilde => "~".to_string(),
            Ascii::DoubleTilde => "≈".to_string(),
        }
    }
}

impl From<&str> for Ascii {
    fn from(value: &str) -> Self {
        match value {
            " " => Ascii::Space,
            "+" => Ascii::Plus,
            "~" => Ascii::Tilde,
            "≈" => Ascii::DoubleTilde,
            _ => Ascii::Space,
        }
    }
}

impl<'de> Deserialize<'de> for Ascii {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AsciiVisitor;

        impl<'de> Visitor<'de> for AsciiVisitor {
            type Value = Ascii;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Unable to parse to Ascii")
            }

            fn visit_str<E>(self, value: &str) -> Result<Ascii, E>
            where
                E: de::Error,
            {
                Ok(Ascii::from(value))
            }
        }

        deserializer.deserialize_str(AsciiVisitor)
    }
}
