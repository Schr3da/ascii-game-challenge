use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Frame;

pub fn draw_menu<B: Backend>(f: &mut Frame<B>) {
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

    let title = Paragraph::new(Spans::from("")).alignment(Alignment::Center);

    f.render_widget(title, parent_layout[0]);

    let list_layout = Layout::default()
        .margin(0)
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(parent_layout[1]);

    let items = [
        ListItem::new("1. New Game").style(Style::default().bg(Color::Red)),
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
        .style(Style::default().fg(Color::Black));

    f.render_widget(list, list_layout[1]);
}
