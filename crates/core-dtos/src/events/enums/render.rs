#[derive(Clone)]
pub enum RenderEvents {
    OnWorldWillUpdate,
}

#[derive(Clone)]
pub enum RenderSubscription {
    OnWorldDidUpdate,
}
