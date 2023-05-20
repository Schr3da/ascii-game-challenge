use bevy_ecs::prelude::Component;

use crate::prelude::*;

#[derive(Clone, Component)]
pub enum UiViewChild {
    List(UiList),
    Label(UiLabel),
}

#[derive(Clone, Component)]
pub struct UiView {
    pub id: UiViewIds,
    pub state: UiViewState,
    pub children: Vec<UiViewChild>,
}
