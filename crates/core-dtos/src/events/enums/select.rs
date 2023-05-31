use tsify::Tsify;
use serde::{Serialize, Deserialize};

#[derive(Clone, Tsify, Serialize, Deserialize)]
pub enum SelectionDirections {
    Next,
    Previous,
}
