use crate::prelude::*;

#[derive(Clone)]
pub enum UiEvents {
    OnClick(ViewComponentIds),
}

#[derive(Clone)]
pub enum UiSubscription {}
