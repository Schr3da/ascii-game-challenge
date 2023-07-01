use core_dtos::prelude::SelectedCell;
use tui::backend::Backend;
use tui::style::{Color, Style};
use tui::widgets::Block;
use tui::Frame;

pub fn render_selected_cell<B: Backend>(context: &mut Frame<B>, cell: &Option<SelectedCell>) {
    let next = match cell {
        Some(d) => d,
        None => return,
    };
    
    let size = context.size();

    let tile_frame = tui::layout::Rect {
        x: next.frame.x as u16,
        y: next.frame.y as u16 + next.top,
        width: next.frame.width as u16,
        height: next.frame.height as u16,
    };

    if !size.intersects(tile_frame) {
        return;
    }

    let block = Block::default()
        .title("S".to_string())
        .style(Style::default().bg(Color::Green).fg(Color::Black));

    context.render_widget(block, tile_frame);
}
