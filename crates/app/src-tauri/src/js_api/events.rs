use core_dtos::prelude::SendEvents;

#[allow(dead_code)]
pub enum WebViewEvents {
  OnDidMount,
  OnDidSubscribe,
  OnSendEvent(SendEvents),
}