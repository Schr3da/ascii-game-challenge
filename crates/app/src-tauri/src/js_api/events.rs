use core_dtos::prelude::*;

pub enum WebViewEvents {
  OnDidMount,
  OnDidSubscribe,
  OnEcsEvent(SendEvents),
}