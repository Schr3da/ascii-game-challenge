use core_dtos::prelude::SelectedCell;
use tui::backend::Backend;
use tui::style::Style;
use tui::widgets::Block;
use tui::Frame;

use crate::export::prelude::*;

pub fn render_selected_cell<B: Backend>(context: &mut Frame<B>, cell: &Option<SelectedCell>) {
    let next = match cell {
        Some(d) => d,
        None => return,
    };

    let cell = &next.cell;
    let frame = &next.frame;

    let size = context.size();

    let tile_frame = tui::layout::Rect {
        x: frame.x as u16,
        y: frame.y as u16 + next.top,
        width: frame.width as u16,
        height: frame.height as u16,
    };

    if !size.intersects(tile_frame) {
        return;
    }

    let background = to_terminal_color(&cell.background);
    let foreground = to_terminal_color(&cell.foreground);

    let block = Block::default()
        .title(cell.symbol.to_string())
        .style(Style::default().bg(background).fg(foreground));

    context.render_widget(block, tile_frame);
}
