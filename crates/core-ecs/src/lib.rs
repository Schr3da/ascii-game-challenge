mod core;
mod resources;
mod systems;
mod views;

pub mod prelude {
    pub use super::core::*;
    pub use super::resources::prelude::*;
    pub(crate) use super::systems::prelude::*;
    pub(crate) use super::views::prelude::*;
}
