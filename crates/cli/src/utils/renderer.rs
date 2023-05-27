use core_dtos::prelude::*;
use tui::backend::Backend;
use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Frame;

use crate::export::prelude::*;

pub fn render_view<B: Backend>(
    context: &mut Frame<B>,
    root_layout: tui::layout::Rect,
    view: &UiView,
) {
    let view_layout = generate_layout(&view, root_layout);

    let selected_id = view.state.selected_id.clone();

    view.children.iter().enumerate().for_each(|(i, c)| {
        match &c {
            UiViewChild::Section(v) => {
                render_view(context, view_layout[i], v);
            }
            UiViewChild::Label(l) => {
                let alignment = match l.alignment {
                    TextAlignment::Center => Alignment::Center,
                    TextAlignment::Left => Alignment::Left,
                    TextAlignment::Right => Alignment::Right,
                };

                let title = Paragraph::new(Spans::from(l.text.clone())).alignment(alignment);
                context.render_widget(title, view_layout[i]);
            }
            UiViewChild::List(l) => {
                let items: Vec<ListItem> = l
                    .children
                    .iter()
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
                            .title(l.label.clone())
                            .title_alignment(Alignment::Center)
                            .borders(Borders::ALL),
                    )
                    .style(Style::default().fg(Color::Black));

                context.render_widget(list, view_layout[i]);
            }
            UiViewChild::Placeholder => {
                let block =
                    Block::default().style(Style::default().bg(Color::Black).fg(Color::White));
                context.render_widget(block, view_layout[i]);
            },

            UiViewChild::GameCanvas=> {
                let block =
                    Block::default().style(Style::default().bg(Color::White).fg(Color::White));
                context.render_widget(block, view_layout[i]);
            }
        };
    });
}
