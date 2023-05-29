use bevy_ecs::prelude::Component;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Component, Serialize, Deserialize)]
pub enum UiViewChild {
    List(UiList),
    Label(UiLabel),
    Section(UiView),
    Placeholder,
    GameCanvas(Rect),
}

#[derive(Debug, PartialEq, Eq, Clone, Component, Serialize, Deserialize)]
pub struct UiView {
    pub id: UiViewIds,
    pub layout: UiLayout,
    pub state: UiViewState,
    pub children: Vec<UiViewChild>,
}
