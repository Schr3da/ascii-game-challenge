mod on_click;
mod on_close_view;
mod on_select;
mod on_select_by_id;
mod on_shortcut;
mod on_ui_did_update;
mod on_open_popup;
mod on_close_popup;
mod scheduler;

pub mod prelude {
    pub(crate) use super::on_click::*;
    pub(crate) use super::on_close_view::*;
    pub(crate) use super::on_select::*;
    pub(crate) use super::on_select_by_id::*;
    pub(crate) use super::on_shortcut::*;
    pub(crate) use super::on_ui_did_update::*;
    pub(crate) use super::on_open_popup::*;
    pub(crate) use super::on_close_popup::*;
    pub use super::scheduler::*;
}
