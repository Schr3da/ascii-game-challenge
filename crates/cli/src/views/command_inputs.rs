use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Clear};
use tui::Frame;

use core_dtos::prelude::*;

use crate::export::prelude::*;

pub fn render_command_inputs_view<B: Backend>(context: &mut Frame<B>, view: &UiPopupView) {
    let screen_size = context.size();

    let size = tui::layout::Rect::new(
        screen_size.width - (screen_size.width / 4) as u16,
        screen_size.y,
        (screen_size.width / 4) as u16,
        (screen_size.height) as u16,
    );

    context.render_widget(Clear, size);

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::White).fg(Color::Black));
    context.render_widget(block, size);

    let root_layout = Layout::default()
        .margin(0)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    render_popup_view(context, root_layout[0], &view);
}
