use tokio::sync::mpsc::error::TryRecvError;

use core_dtos::prelude::*;
use core_state::prelude::*;

pub fn subscription_handler(
    event: Result<SubscriptionEvents, TryRecvError>,
    _state: &mut AppState,
) -> bool {
    let unwrapped_event = match event {
        Ok(e) => e,
        Err(_) => return true,
    };

    let next = match unwrapped_event {
        SubscriptionEvents::General(e) => e,
        SubscriptionEvents::Ui(_) => return true,
        SubscriptionEvents::Renderer(_) => return true,
    };

    match next {
        GeneralSubscription::OnApplicationDidInitialise => true,
        GeneralSubscription::OnApplicationDidClose => false,
    }
}
