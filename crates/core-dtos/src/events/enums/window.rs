use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Tsify)]
pub enum WindowEvents {
    Resize(u16, u16),
}
