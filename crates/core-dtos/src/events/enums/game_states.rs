use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize, Tsify)]
pub enum GameStatus {
    #[default]
    GameDidNotStart,
    GameDidStart,
    GameDidPaused,
    GameWillEnded,
}
