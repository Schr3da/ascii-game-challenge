use bevy_ecs::prelude::Component;
use core_dtos::prelude::*;

use crate::prelude::*;

#[derive(Component)]
pub enum UiViewChild {
    List(UiList),
    Label(UiLabel),
}

#[derive(Component)]
pub struct UiView {
    pub id: UiViewIds,
    pub state: UiViewState,
    pub children: Vec<UiViewChild>,
}