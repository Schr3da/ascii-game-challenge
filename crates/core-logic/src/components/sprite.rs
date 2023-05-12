use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Sprite {
    pub asset: Asset,
    pub frame: Rect,
}