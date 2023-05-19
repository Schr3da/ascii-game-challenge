use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

#[derive(Default, Component)]
pub struct UiStore{
  pub current_view: UiViewIds,
}
