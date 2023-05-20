use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
