use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub enum WindowEvents {
    Resize(u16, u16),
}
