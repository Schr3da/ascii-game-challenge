use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Tsify)]
pub struct GameMeta {
    pub status: GameStatus,
    pub cursor: Option<SelectedCell>,
}
