use std::collections::HashMap;
use tui::style::{Modifier, Style};
use tui::text::Span;
use tui::widgets::Block;

use core_logic::prelude::*;
use core_serde::prelude::*;

use crate::export::prelude::*;

#[derive(Default)]
pub struct AssetManager {
    pub tile_cache: HashMap<Ascii, Block<'static>>,
}

impl AssetManager {
    pub fn load_tile(&mut self, filename: String) {
        let data = load_json_from_file::<Cell>(&filename);

        let title_style = match data.is_bold {
            true => Style::default().add_modifier(Modifier::BOLD),
            false => Style::default(),
        };

        let title = Span::styled(data.symbol.to_string(), title_style);

        let next = Block::default().title(title).style(
            Style::default()
                .bg(_to_terminal_color(data.background))
                .fg(_to_terminal_color(data.foreground)),
        );

        self.tile_cache.insert(data.symbol, next);
    }
}
