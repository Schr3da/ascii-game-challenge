use tui::layout::{Constraint, Direction, Layout};

use core_dtos::prelude::*;

pub fn generate_layout(view: &UiView, size: tui::layout::Rect) -> Vec<tui::layout::Rect> {
    let layout = &view.layout;

    let margin = layout.margin;

    let direction = match layout.alignment {
        LayoutAlignments::Horizontal => Direction::Horizontal,
        LayoutAlignments::Vertical => Direction::Vertical,
    };

    let constraints: Vec<Constraint> = layout
        .constraints
        .iter()
        .map(|c| match c {
            LayoutConstraints::Percentage(v) => Constraint::Percentage(v.clone()),
        })
        .collect();

    Layout::default()
        .margin(margin)
        .direction(direction)
        .constraints(constraints)
        .split(size)
}
