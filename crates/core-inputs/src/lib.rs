mod mouse_inputs;
mod keyboard_inputs;
mod window_inputs;

pub mod prelude {
    pub use super::mouse_inputs::*;
    pub use super::keyboard_inputs::*;
    pub use super::window_inputs::*;
}
