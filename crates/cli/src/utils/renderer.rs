use core_dtos::prelude::*;
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Frame;

use crate::export::prelude::*;

fn render_label<B: Backend>(context: &mut Frame<B>, label: &UiLabel, size: Rect) {
    let alignment = match label.alignment {
        TextAlignment::Center => Alignment::Center,
        TextAlignment::Left => Alignment::Left,
        TextAlignment::Right => Alignment::Right,
    };

    let title = Paragraph::new(Spans::from(label.text.clone())).alignment(alignment);
    context.render_widget(title, size);
}

fn render_list<B: Backend>(
    context: &mut Frame<B>,
    list: &UiList,
    selected_id: &ViewComponentIds,
    size: Rect,
) {
    let items: Vec<ListItem> = list
        .children
        .iter()
        .enumerate()
        .map(|(index, label)| {
            let text = format!(" {}. {}", index + 1, label.text);

            match selected_id == &label.id {
                true => ListItem::new(text).style(Style::default().bg(Color::Red).fg(Color::White)),
                false => ListItem::new(text),
            }
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(list.label.clone())
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::Black));

    context.render_widget(list, size);
}

fn render_placeholder<B: Backend>(context: &mut Frame<B>, size: Rect) {
    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    context.render_widget(block, size);
}

fn render_canvas<B: Backend>(context: &mut Frame<B>, frame: &core_dtos::prelude::Rect) {
    for y in frame.y..frame.height {
        for x in 0..frame.width {
            let block = Block::default()
                .title("~")
                .style(Style::default().bg(Color::Gray).fg(Color::Black));

            context.render_widget(
                block,
                Rect {
                    x: x as u16,
                    y: y as u16,
                    width: 1,
                    height: 1,
                },
            );
        }
    }
}

pub fn render_view<B: Backend>(context: &mut Frame<B>, root_layout: Rect, view: &UiView) {
    let view_layout = generate_layout(&view, root_layout);

    let selected_id = view.state.selected_id.clone();

    view.children.iter().enumerate().for_each(|(i, c)| {
        match &c {
            UiViewChild::Section(v) => render_view(context, view_layout[i], v),
            UiViewChild::Label(l) => render_label(context, l, view_layout[i]),
            UiViewChild::List(l) => render_list(context, l, &selected_id, view_layout[i]),
            UiViewChild::Placeholder => render_placeholder(context, view_layout[i]),
            UiViewChild::GameCanvas(frame) => render_canvas(context, frame),
        };
    });
}
