use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Frame;

use core_dtos::prelude::*;

pub fn draw_menu<B: Backend>(f: &mut Frame<B>, data: UiView) {
    let size = f.size();

    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
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

    let list_layout = Layout::default()
        .margin(1)
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(parent_layout[1]);

    let selected_id = data.state.selected_id;

    data.children.into_iter().for_each(|c| {
        match c {
            UiViewChild::Label(l) => {
                let title = Paragraph::new(Spans::from(l.text)).alignment(Alignment::Center);
                f.render_widget(title, parent_layout[0]);
            }
            UiViewChild::List(l) => {
                let items: Vec<ListItem> = l
                    .children
                    .into_iter()
                    .enumerate()
                    .map(|(index, label)| {
                        let text = format!(" {}. {}", index + 1, label.text);

                        match selected_id == label.id {
                            true => ListItem::new(text)
                                .style(Style::default().bg(Color::Red).fg(Color::White)),
                            false => ListItem::new(text),
                        }
                    })
                    .collect();

                let list = List::new(items)
                    .block(
                        Block::default()
                            .title("Main Menu")
                            .title_alignment(Alignment::Center)
                            .borders(Borders::ALL),
                    )
                    .style(Style::default().fg(Color::Black));

                f.render_widget(list, list_layout[1]);
            }
        };
    });
}
