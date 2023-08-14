use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Tsify)]
#[serde(rename_all = "camelCase")]
pub enum AsciiIds {
    Sand,
    NotVisible,
    ShallowWater,
    DeepWater,
    UnknownAsciiId,
}

impl Default for AsciiIds {
    fn default() -> Self {
        AsciiIds::UnknownAsciiId
    }
}
