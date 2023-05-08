use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum CellColors {
    Black,
    White,
}

impl Default for CellColors {
    fn default() -> Self {
        CellColors::Black
    }
}
