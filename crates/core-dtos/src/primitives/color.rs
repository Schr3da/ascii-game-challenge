use serde::{Deserialize, Serialize};

use tsify::Tsify;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Tsify)]
#[serde(rename_all = "camelCase")]
pub enum CellColors {
    Black,
    White,
    Blue,
    LightBlue,
}

impl Default for CellColors {
    fn default() -> Self {
        CellColors::Black
    }
}
