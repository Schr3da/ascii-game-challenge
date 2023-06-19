use serde::{Deserialize, Serialize};

use tsify::Tsify;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Tsify)]
#[serde(rename_all = "camelCase")]
pub enum CellColors {
    Rgb(u8, u8, u8),
}

impl Default for CellColors {
    fn default() -> Self {
        CellColors::Rgb(0, 0, 0)
    }
}
