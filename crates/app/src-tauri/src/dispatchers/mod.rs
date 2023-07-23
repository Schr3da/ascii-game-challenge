mod popup_dispatcher;
mod view_dispatcher;
mod general_dispatcher;

pub mod prelude {
  pub use super::general_dispatcher::*;
  pub use super::view_dispatcher::*;
  pub use super::popup_dispatcher::*;
}