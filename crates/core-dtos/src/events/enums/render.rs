use crate::prelude::*;

#[derive(Clone)]
pub enum RenderEvents {
    OnWorldWillUpdate,
}

#[derive(Clone, Eq, PartialEq)]
pub enum RenderSubscription {
    OnWorldDidUpdate(Option<UiView>),
}
