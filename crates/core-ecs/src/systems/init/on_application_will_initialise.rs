use bevy_ecs::prelude::*;

use crate::prelude::*;

pub fn on_application_will_initialise_system(mut commands: Commands) {
    commands.spawn(UiStore::default());
    commands.spawn(main_view());
    commands.spawn(options_view());
}
