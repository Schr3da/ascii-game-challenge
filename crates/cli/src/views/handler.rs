use tui::{backend::Backend, Frame};

use core_dtos::prelude::*;

use crate::export::prelude::*;

pub fn draw_view_to_terminal_handler<B: Backend>(context: &mut Frame<B>, view: &Option<UiView>) {
    let next = match view {
        Some(v) => v,
        None => return,
    };

    match &next.id {
        UiViewIds::Main => _ = render_menu(context, next),
        UiViewIds::Options => _ = render_options(context, next),
        UiViewIds::Game => _ = render_game(context, next),
        UiViewIds::Popup(p) => match p {
            UiPopupViewIds::Buildings | UiPopupViewIds::Actions | UiPopupViewIds::Logger => {
                _ = render_popup(context, next)
            }
        },
        UiViewIds::Quit => return,
    };
}

pub fn draw_cursor_to_terminal_handler<B: Backend>(
    context: &mut Frame<B>,
    tile: &Option<SelectedCell>,
) {
    render_selected_cell(context, tile);
}
