use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Frame;

use crate::export::prelude::*;

pub fn draw_menu<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();

    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(block, size);

    let chunks = Layout::default()
        .margin(2)
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(1), Constraint::Max(1)].as_ref())
        .split(size);

    let title = Paragraph::new(Spans::from("Ascii Empire")).alignment(Alignment::Center);

    f.render_widget(title, chunks[0]);

    let items = [
        ListItem::new("1. New Game"),
        ListItem::new("2. Options"),
        ListItem::new("3. Quit"),
    ];

    let list = List::new(items)
        .block(
            Block::default()
                .title("Main Menu")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::Black))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    f.render_widget(list, rect_center(chunks[1], 26, 5));
}
