use bevy_ecs::prelude::Component;

use crate::prelude::*;

#[derive(Debug, Clone, Component)]
pub enum UiViewChild {
    List(UiList),
    Label(UiLabel),
}

#[derive(Debug, Clone, Component)]
pub struct UiView {
    pub id: UiViewIds,
    pub state: UiViewState,
    pub children: Vec<UiViewChild>,
}
