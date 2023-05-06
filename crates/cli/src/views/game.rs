use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::Block;
use tui::Frame;

use crate::export::prelude::*;

pub fn draw_game<B: Backend>(f: &mut Frame<B>, _state: &AppState) {
    let size = f.size();

    let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::Black));
    f.render_widget(block, size);

    let tile = Block::default()
        .title(Span::styled(
            " + ",
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(Color::Blue).fg(Color::White));
    f.render_widget(tile.clone(), Rect::new(0, 0, 3, 1));

    f.render_widget(tile.clone(), Rect::new(0, 4, 3, 1));
}
