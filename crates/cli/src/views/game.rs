use tui::backend::Backend;
use tui::style::{Color, Style};
use tui::widgets::Block;
use tui::Frame;

use crate::export::prelude::*;

pub fn draw_game<B: Backend>(f: &mut Frame<B>, _state: &AppState, _assets: &AssetManager) {
    let size = f.size();

    let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::Black));
    f.render_widget(block, size);
}
