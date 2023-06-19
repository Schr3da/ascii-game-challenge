use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
pub enum AsciiIds {
    Sand,
    ShallowWater,
    DeepWater,
    UnknownAsciiId,
}

impl Default for AsciiIds {
    fn default() -> Self {
        AsciiIds::UnknownAsciiId
    }
}
