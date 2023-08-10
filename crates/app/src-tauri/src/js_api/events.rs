use core_dtos::prelude::*;

#[derive(Debug)]
pub enum WebViewEvents {
    OnDidMount(u16, u16),
    OnDidSubscribe,
    OnEcsEvent(SendEvents),
    OnInputEvent(InputEvents),
}
