mod init_scheduler;
mod init_systems;

pub mod prelude {
    pub use super::init_systems::*;
    pub use super::init_scheduler::*;
}
