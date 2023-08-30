use serde::{Deserialize, Serialize};
use tsify::Tsify;
use crate::prelude::*;

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize, Tsify)]
pub enum GameStatus {
    #[default]
    GameDidNotStart,
    GameDidStart,
    GameDidPaused,
    GameWillEnded,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Tsify)]
pub struct GameMeta {
    pub status: GameStatus,
    pub cursor: Option<SelectedCell>,
}