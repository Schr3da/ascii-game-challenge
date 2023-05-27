use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::Block;
use tui::Frame;

use crate::export::prelude::*;
use core_dtos::prelude::*;

pub fn render_game<B: Backend>(context: &mut Frame<B>, view: &UiView) {
    let size = context.size();

    let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::White));
    context.render_widget(block, size);

    let root_layout = Layout::default()
        .horizontal_margin(2)
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    render_view(context, root_layout[0], view);
}