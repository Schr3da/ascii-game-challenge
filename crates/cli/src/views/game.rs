use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, Paragraph};
use tui::Frame;

use core_dtos::prelude::*;

pub fn draw_game<B: Backend>(f: &mut Frame<B>, data: &UiView) {
    let size = f.size();

    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::White));
    f.render_widget(block, size);

    let parent_layout = Layout::default()
        .margin(0)
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(size);

    data.children.iter().for_each(|c| {
        match c {
            UiViewChild::Label(l) => {
                let title =
                    Paragraph::new(Spans::from(l.text.clone())).alignment(Alignment::Center);
                f.render_widget(title, parent_layout[0]);
            }
            _ => {}
        };
    });
}
