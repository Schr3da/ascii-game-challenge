use crate::prelude::*;

#[derive(Clone)]
pub enum UiEvents {
    OnSelect(SelectionDirections),
    OnClick(ViewComponentIds),
}

#[derive(Clone)]
pub enum UiSubscription {}
