use core_dtos::prelude::*;
use tui::backend::Backend;
use tui::layout;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Frame;

use super::colors_utils::*;

pub struct RenderUtils;

impl RenderUtils {
    pub fn create_layout(layout: &UiLayout, size: tui::layout::Rect) -> Vec<tui::layout::Rect> {
        let margin = layout.margin;

        let direction = match layout.alignment {
            LayoutAlignments::Horizontal => Direction::Horizontal,
            LayoutAlignments::Vertical => Direction::Vertical,
        };

        let constraints: Vec<Constraint> = layout
            .constraints
            .iter()
            .map(|c| match c.clone() {
                LayoutConstraints::Percentage(v) => Constraint::Percentage(v),
                LayoutConstraints::MinValue(v) => Constraint::Min(v),
                LayoutConstraints::MaxValue(v) => Constraint::Max(v),
                LayoutConstraints::Value(v) => Constraint::Length(v),
            })
            .collect();

        Layout::default()
            .margin(margin)
            .direction(direction)
            .constraints(constraints)
            .split(size)
    }

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

        let title = label.format_with_data(view_data);

        let paragraph = Paragraph::new(Spans::from(title.as_str())).alignment(alignment);
        context.render_widget(paragraph, size);
    }

    fn render_list<B: Backend>(
        context: &mut Frame<B>,
        list: &UiList,
        selected_id: &ViewComponentIds,
        size: layout::Rect,
    ) {
        let items: Vec<ListItem> =
            list.children
                .iter()
                .enumerate()
                .map(|(_, label)| {
                    let title = label.format();

                    match selected_id == &label.id {
                        true => ListItem::new(title)
                            .style(Style::default().bg(Color::Red).fg(Color::White)),
                        false => ListItem::new(title),
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
            let background = ColorUtils::to_terminal_color(&cell.background);
            let foreground = ColorUtils::to_terminal_color(&cell.foreground);

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

    pub fn render_view<B: Backend>(
        context: &mut Frame<B>,
        root_layout: layout::Rect,
        view: &UiView,
    ) {
        let view_layout = Self::create_layout(&view.layout, root_layout);

        let selected_id = &view.state.selected_id;

        let view_data = &view.state.view_data;

        view.children.iter().enumerate().for_each(|(i, c)| {
            match &c {
                UiViewChild::Section(v) => Self::render_view(context, view_layout[i], v),
                UiViewChild::Label(l) => Self::render_label(context, l, view_data, view_layout[i]),
                UiViewChild::List(l) => Self::render_list(context, l, &selected_id, view_layout[i]),
                UiViewChild::Placeholder => Self::render_placeholder(context, view_layout[i]),
                UiViewChild::GameCanvas(data) => {
                    Self::render_canvas(context, data);
                }
            };
        });
    }

    pub fn render_popup_view<B: Backend>(
        context: &mut Frame<B>,
        root_layout: layout::Rect,
        view: &UiView,
    ) {
        let view_layout = Self::create_layout(&view.layout, root_layout);

        let selected_id = &view.state.selected_id;

        let view_data = &view.state.view_data;

        view.children.iter().enumerate().for_each(|(i, c)| {
            match &c {
                UiViewChild::Section(v) => Self::render_view(context, view_layout[i], v),
                UiViewChild::Label(l) => Self::render_label(context, l, view_data, view_layout[i]),
                UiViewChild::List(l) => Self::render_list(context, l, &selected_id, view_layout[i]),
                _ => return,
            };
        });
    }
}
