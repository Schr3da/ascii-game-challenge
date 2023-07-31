mod general_dispatcher;
mod meta;
mod popup_dispatcher;
mod view_dispatcher;

pub mod prelude {
    pub use super::general_dispatcher::*;
    pub use super::meta::*;
    pub use super::popup_dispatcher::*;
    pub use super::view_dispatcher::*;
}
