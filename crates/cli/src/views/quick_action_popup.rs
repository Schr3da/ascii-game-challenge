use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Clear};
use tui::Frame;

use core_dtos::prelude::*;

use crate::export::prelude::*;

pub fn render_quick_action_popup<B: Backend>(context: &mut Frame<B>, view: &UiView) {
    let screen_size = context.size();

    let size = tui::layout::Rect::new(0, screen_size.height - 1, screen_size.width as u16 - 1, 1);

    context.render_widget(Clear, size);

    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    context.render_widget(block, size);

    let root_layout = Layout::default()
        .margin(0)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    render_popup_view(context, root_layout[0], &view);
}
