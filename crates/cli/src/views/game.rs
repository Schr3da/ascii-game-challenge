use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::Block;
use tui::Frame;

use core_dtos::prelude::*;

pub fn draw_game<B: Backend>(f: &mut Frame<B>, _: &UiView) {
    let size = f.size();

    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::White));
    f.render_widget(block, size);
}
