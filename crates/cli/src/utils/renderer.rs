use std::collections::HashMap;

use core_dtos::prelude::*;
use tui::backend::Backend;
use tui::layout;
use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Frame;

use crate::export::prelude::*;

fn render_label<B: Backend>(
    context: &mut Frame<B>,
    label: &UiLabel,
    view_data: &ViewDataTypes,
    size: layout::Rect,
) {
    let alignment = match label.alignment {
        TextAlignment::Center => Alignment::Center,
        TextAlignment::Left => Alignment::Left,
        TextAlignment::Right => Alignment::Right,
    };

    let data: HashMap<ViewComponentIds, String> = view_data.into();

    let title = match data.get(&label.id) {
        Some(v) => v,
        None => &label.text,
    };

    let paragraph = Paragraph::new(Spans::from(title.as_str())).alignment(alignment);
    context.render_widget(paragraph, size);
}

fn render_list<B: Backend>(
    context: &mut Frame<B>,
    list: &UiList,
    selected_id: &ViewComponentIds,
    size: layout::Rect,
) {
    let items: Vec<ListItem> = list
        .children
        .iter()
        .enumerate()
        .map(|(_, label)| {
            let text = format!(" {}", label.text);

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

fn render_placeholder<B: Backend>(context: &mut Frame<B>, size: layout::Rect) {
    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    context.render_widget(block, size);
}

fn render_canvas<B: Backend>(context: &mut Frame<B>, cells: &Vec<(Cell, Position)>) {
    let size = context.size();

    for (cell, position) in cells {
        let background = to_terminal_color(&cell.background);
        let foreground = to_terminal_color(&cell.foreground);

        let block = Block::default()
            .title(cell.symbol.to_string())
            .style(Style::default().bg(background).fg(foreground));

        let next = layout::Rect {
            x: position.x as u16,
            y: position.y as u16,
            width: 1,
            height: 1,
        };

        if !next.intersects(size) {
            continue;
        }

        context.render_widget(block, next);
    }
}

pub fn render_view<B: Backend>(context: &mut Frame<B>, root_layout: layout::Rect, view: &UiView) {
    let view_layout = generate_layout(&view.layout, root_layout);

    let selected_id = &view.state.selected_id;

    let view_data = &view.state.view_data;

    view.children.iter().enumerate().for_each(|(i, c)| {
        match &c {
            UiViewChild::Section(v) => render_view(context, view_layout[i], v),
            UiViewChild::Label(l) => render_label(context, l, view_data, view_layout[i]),
            UiViewChild::List(l) => render_list(context, l, &selected_id, view_layout[i]),
            UiViewChild::Placeholder => render_placeholder(context, view_layout[i]),
            UiViewChild::GameCanvas(data) => {
                render_canvas(context, data);
            }
        };
    });
}

pub fn render_popup_view<B: Backend>(
    context: &mut Frame<B>,
    root_layout: layout::Rect,
    view: &UiView,
) {
    let view_layout = generate_layout(&view.layout, root_layout);

    let selected_id = &view.state.selected_id;

    let view_data = &view.state.view_data;

    view.children.iter().enumerate().for_each(|(i, c)| {
        match &c {
            UiViewChild::Section(v) => render_view(context, view_layout[i], v),
            UiViewChild::Label(l) => render_label(context, l, view_data, view_layout[i]),
            UiViewChild::List(l) => render_list(context, l, &selected_id, view_layout[i]),
            _ => return,
        };
    });
}
