use serde::{Deserialize, Serialize};
use tsify::*;

use crate::prelude::SelectedCell;

#[derive(Debug, Default, Clone, Serialize, Deserialize, Tsify, Eq, PartialEq)]
pub struct PopupState {
    #[serde(rename(
        serialize = "currentSelectedGameTile",
        deserialize = "currentSelectedGameTile"
    ))]
    pub current_selected_game_tile: SelectedCell,
}
