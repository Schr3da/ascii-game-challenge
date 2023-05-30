use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize, Tsify)]
pub struct Sprite {
    pub asset: Asset,
    pub frame: Rect,
}
