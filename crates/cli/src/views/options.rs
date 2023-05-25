use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::Block;
use tui::Frame;

use core_dtos::prelude::*;

use crate::export::prelude::render_view;

pub fn render_options<B: Backend>(context: &mut Frame<B>, view: &UiView) {
    let size = context.size();

    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::White));
    context.render_widget(block, size);

    let root_layout = Layout::default()
        .margin(0)
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(size);

    render_view(context, root_layout[1], view);
}
