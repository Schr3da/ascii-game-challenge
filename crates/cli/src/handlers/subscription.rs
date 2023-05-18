use core_dtos::prelude::*;

pub fn subscription_handler(event: Option<SubscriptionEvents>) -> bool {
    let unwrapped_event = match event {
        Some(e) => e,
        None => return true,
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
