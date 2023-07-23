use tokio::sync::mpsc::Sender;

use core_dtos::prelude::*;

pub struct SubscriberConfig {
  pub renderer_config: RendererSubscriptionConfig,
  pub sender: Sender<EcsEvents>,
}