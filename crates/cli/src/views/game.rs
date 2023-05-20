use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::Block;
use tui::Frame;

use crate::export::prelude::*;

use core_dtos::prelude::*;
use core_ecs::prelude::*;

fn _draw_cells<B: Backend>(f: &mut Frame<B>, data: &Vec<Cell>, height: usize) {
    data.iter().enumerate().for_each(|(index, d)| {
        let style = match d.is_bold {
            true => Style::default().add_modifier(Modifier::BOLD),
            false => Style::default(),
        };

        let title = Span::styled(d.symbol.to_string(), style);

        let next = Block::default().title(title).style(
            Style::default()
                .bg(_to_terminal_color(&d.background))
                .fg(_to_terminal_color(&d.foreground)),
        );

        let rect: Rect = Rect::new(index as u16, height as u16, 1, 1);

        f.render_widget(next, rect);
    });
}

pub fn _draw_game<B: Backend>(f: &mut Frame<B>, assets: &AssetResources) {
    let size = f.size();

    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::White));
    f.render_widget(block, size);
    //
    assets
        .asset_cache
        .iter()
        .enumerate()
        .for_each(|(_index, (_, d))| {
            _draw_cells(f, &d.cells, 0);
            _draw_cells(f, &d.cells, 1);
            _draw_cells(f, &d.cells, 2);
        });
}
