mod application_dispatcher;
mod game_dispatcher;
mod popup_dispatcher;
mod view_dispatcher;

pub mod prelude {
    pub use super::application_dispatcher::*;
    pub use super::game_dispatcher::*;
    pub use super::popup_dispatcher::*;
    pub use super::view_dispatcher::*;
}
