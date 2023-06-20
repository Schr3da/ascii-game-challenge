use bevy_ecs::prelude::Component;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Component, Serialize, Deserialize, Tsify)]
pub enum UiViewChild {
    List(UiList),
    Label(UiLabel),
    Section(UiView),
    Placeholder,
    GameCanvas(Vec<(Cell, Position)>),
}

#[derive(Debug, PartialEq, Eq, Clone, Component, Serialize, Deserialize, Tsify)]
pub struct UiView {
    pub id: UiViewIds,
    pub layout: UiLayout,
    pub state: UiViewState,
    pub children: Vec<UiViewChild>,
}

#[derive(Debug, PartialEq, Eq, Clone, Component, Serialize, Deserialize, Tsify)]
pub struct UiPopupView{
    pub id: UiPopupViewIds,
    pub layout: UiLayout,
    pub state: UiViewState,
    pub children: Vec<UiViewChild>,
}
