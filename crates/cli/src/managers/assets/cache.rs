use std::collections::HashMap;
use tui::style::{Modifier, Style};
use tui::text::Span;
use tui::widgets::Block;

use core_logic::prelude::*;
use core_serde::prelude::*;

use crate::export::prelude::*;

pub struct AssetManager {
    pub cell_cache: HashMap<Ascii, Block<'static>>,
    pub config: AssetConfig,
}

impl Default for AssetManager {
    fn default() -> Self {
        let cell_cache = HashMap::new();
        let config = load_json_from_file::<AssetConfig>("./assets/root_config.json");

        AssetManager { cell_cache, config }
    }
}

impl AssetManager {
    pub fn load_cells(&mut self) {
        let data = load_json_from_file::<Vec<Cell>>(&self.config.cells);

        for d in data {
            let cell = match d.is_bold {
                true => Style::default().add_modifier(Modifier::BOLD),
                false => Style::default(),
            };

            let title = Span::styled(d.symbol.to_string(), cell);

            let next = Block::default().title(title).style(
                Style::default()
                    .bg(_to_terminal_color(d.background))
                    .fg(_to_terminal_color(d.foreground)),
            );

            self.cell_cache.insert(d.symbol, next);
        }
    }
}
