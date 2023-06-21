use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Clone, Tsify, Serialize, Deserialize)]
pub enum SelectionDirections {
    Next,
    Previous,
}
