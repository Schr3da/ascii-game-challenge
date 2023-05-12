use core_logic::prelude::prelude::AssetResources;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::Block;
use tui::Frame;

use crate::export::prelude::*;

use core_logic::prelude::*;

fn draw_cells<B: Backend>(f: &mut Frame<B>, data: &Vec<Cell>) {
    data.iter().enumerate().for_each(|(index, d)| {
        let cell = match d.is_bold {
            true => Style::default().add_modifier(Modifier::BOLD),
            false => Style::default(),
        };

        let title = Span::styled(d.symbol.to_string(), cell);

        let next = Block::default().title(title).style(
            Style::default()
                .bg(to_terminal_color(&d.background))
                .fg(to_terminal_color(&d.foreground)),
        );

        let rect: Rect = Rect::new(index as u16, 0, 1, 1);

        f.render_widget(next, rect);
    });
}

pub fn draw_game<B: Backend>(f: &mut Frame<B>, _state: &AppState, assets: &AssetResources) {
    let size = f.size();

    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::White));
    f.render_widget(block, size);

    assets
        .asset_cache
        .iter()
        .enumerate()
        .for_each(|(_index, (_, d))| {
            draw_cells(f, &d.cells);
        });
}
