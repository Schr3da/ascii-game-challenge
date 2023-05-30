use serde::{Deserialize, Serialize};

use tsify::Tsify;

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub enum CellColors {
    Black,
    White,
}

impl Default for CellColors {
    fn default() -> Self {
        CellColors::Black
    }
}
