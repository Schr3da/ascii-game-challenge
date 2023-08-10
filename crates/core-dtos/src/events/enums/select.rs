use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum SelectionDirections {
    Next,
    Previous,
}
