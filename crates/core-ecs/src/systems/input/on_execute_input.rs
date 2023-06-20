use bevy_ecs::prelude::*;

use core_dtos::prelude::*;

use crate::prelude::*;

pub fn on_execute_input_system(subscriber: Res<Subscriber>) {
    let next = match &subscriber.next_event {
        Some(SendEvents::Input(InputEvents::Execute(n))) => n,
        _ => return,
    };

    println!("on_execute_input_system: {:?}", next);
}
