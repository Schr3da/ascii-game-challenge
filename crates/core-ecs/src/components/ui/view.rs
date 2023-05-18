use bevy_ecs::prelude::*;

use crate::prelude::*;

pub enum UiViewChild {
    List(UiList),
    Label(UiLabel),
}

#[derive(Component)]
pub struct UiView {
    pub id: String,
    pub children: Vec<UiViewChild>,
}
