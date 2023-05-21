use crate::prelude::*;

#[derive(Clone)]
pub enum RenderEvents {
    OnWorldWillUpdate,
}

#[derive(Clone)]
pub enum RenderSubscription {
    OnWorldDidUpdate(Option<UiView>, Option<SendEvents>),
}
