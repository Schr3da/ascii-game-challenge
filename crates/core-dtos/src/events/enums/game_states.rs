use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum GameStatus {
    #[default]
    GameDidNotStart,
    GameDidStart,
    GameDidPaused,
    GameWillEnded,
}
