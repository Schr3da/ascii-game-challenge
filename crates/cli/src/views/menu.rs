use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::Block;
use tui::Frame;

use crate::export::prelude::*;

pub fn draw_menu<B: Backend>(f: &mut Frame<B>, _state: &AppState) {
    let size = f.size();
    let _chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(block, size);
}
