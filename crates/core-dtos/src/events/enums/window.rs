use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Clone, Serialize, Deserialize, Tsify)]
pub enum WindowEvents {
    Resize(u16, u16),
}
