use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct UiStore {
    pub previous_view: Vec<UiViewIds>,
    pub current_view: UiViewIds,
    pub current_popup: Option<UiPopupViewIds>,
    pub selected_game_tile: Option<SelectedCell>,
    pub width: u16,
    pub height: u16,
}
