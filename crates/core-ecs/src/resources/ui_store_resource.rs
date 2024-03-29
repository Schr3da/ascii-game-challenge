use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct UiStoreResource {
    pub previous_view: Vec<UiViewIds>,
    pub current_view: UiViewIds,
    pub current_popup: Option<UiPopupViewIds>,
    pub current_game_status: GameStatus,
    pub selected_game_tile: Option<SelectedCell>,
}

impl UiStoreResource {
    pub fn into_meta(&self) -> GameMeta {
        GameMeta {
            status: self.current_game_status.clone(),
            cursor: self.selected_game_tile.clone(),
        }
    }
}
