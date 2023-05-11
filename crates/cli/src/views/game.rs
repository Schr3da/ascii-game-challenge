use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::Block;
use tui::Frame;

use crate::export::prelude::*;

pub fn draw_game<B: Backend>(f: &mut Frame<B>, _state: &AppState, assets: &AssetManager) {
    let size = f.size();

    let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::Black));
    f.render_widget(block, size);

    assets
        .cell_cache
        .iter()
        .enumerate()
        .for_each(|(index, (_, block))| {
            let rect: Rect = Rect::new(index as u16, 0, 1, 1);
            f.render_widget(block.clone(), rect);
        });
}
