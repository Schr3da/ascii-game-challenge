use bevy_ecs::prelude::*;
use core_serde::prelude::*;
use std::collections::HashMap;

use super::prelude::*;
use crate::prelude::*;

#[derive(Resource)]
pub struct AssetResources {
    pub config: AssetConfig,
    pub cell_cache: HashMap<Ascii, Cell>,
    pub asset_cache: HashMap<AssetTypes, Asset>,
}

impl Default for AssetResources {
    fn default() -> Self {
        let config = load_json_from_file::<AssetConfig>("./configs/root_config.json");
        let cell_cache = HashMap::new();
        let asset_cache = HashMap::new();

        AssetResources {
            config,
            asset_cache,
            cell_cache,
        }
    }
}

impl AssetResources {
    fn load_cells(&mut self) {
        let data = load_json_from_file::<Vec<Cell>>(&self.config.cells);

        for d in data {
            self.cell_cache.insert(d.symbol.clone(), d);
        }
    }

    fn load_assets(&mut self) {
        let data = load_json_from_file::<Vec<Asset>>(&self.config.assets);

        for d in data {
            self.asset_cache.insert(d.id.clone(), d);
        }
    }

    pub fn load(&mut self) {
        self.load_cells();
        self.load_assets();
    }
}
