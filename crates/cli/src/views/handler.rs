use tui::{backend::Backend, Frame};

use core_dtos::prelude::*;

use crate::export::prelude::*;

pub fn draw_view_to_terminal_handler<B: Backend>(context: &mut Frame<B>, view: &Option<UiView>) {
    let next = match view {
        Some(v) => v,
        None => return,
    };

    match next.id {
        UiViewIds::Main => _ = render_menu(context, next),
        UiViewIds::Options => _ = render_options(context, next),
        UiViewIds::Game => _ = render_game(context, next),
    };
}

pub fn draw_popup_to_terminal_handler<B: Backend>(
    context: &mut Frame<B>,
    popup: &Option<UiPopupView>,
) {
    let next = match popup {
        Some(p) => p,
        None => return,
    };

    match next.id {
        UiPopupViewIds::Command => _ = render_command_inputs_view(context, next),
    }
}
