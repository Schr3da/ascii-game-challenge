use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_cancel_input_system(subscriber: Res<Subscriber>) {
    match subscriber.next_event {
        Some(SendEvents::Input(InputEvents::Cancel)) => {}
        _ => return,
    };

    println!("cancel input");
}
