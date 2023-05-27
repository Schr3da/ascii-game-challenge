use bevy_ecs::prelude::Component;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Component)]
pub enum UiViewChild {
    List(UiList),
    Label(UiLabel),
    Section(UiView),
    Placeholder,
    GameCanvas,
}

#[derive(Debug, PartialEq, Eq, Clone, Component)]
pub struct UiView {
    pub id: UiViewIds,
    pub layout: UiLayout,
    pub state: UiViewState,
    pub children: Vec<UiViewChild>,
}
