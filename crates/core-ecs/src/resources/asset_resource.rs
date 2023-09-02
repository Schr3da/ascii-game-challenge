use bevy_ecs::prelude::*;
use core_dtos::prelude::*;
use core_serde_utils::prelude::*;
use core_terrain::prelude::Terrain;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AssetConfig {
    pub cells: String,
    pub assets: String,
}

#[derive(Resource)]
pub struct AssetResources {
    pub config: AssetConfig,
    pub cell_cache: HashMap<AsciiIds, Cell>,
    pub asset_cache: HashMap<AssetTypes, Asset>,
    pub terrain: Terrain,
}

impl Default for AssetResources {
    fn default() -> Self {
        let config = SerdeUtils::load_json_file::<AssetConfig>("./configs/root_config.json");
        let cell_cache = HashMap::new();
        let asset_cache = HashMap::new();
        let terrain = Terrain::default();

        AssetResources {
            config,
            asset_cache,
            cell_cache,
            terrain,
        }
    }
}

impl AssetResources {
    fn load_cells(&mut self) {
        let data = SerdeUtils::load_json_file::<Vec<Cell>>(&self.config.cells);

        for d in data {
            self.cell_cache.insert(d.id.clone(), d);
        }
    }

    fn load_assets(&mut self) {
        let data = SerdeUtils::load_json_file::<Vec<Asset>>(&self.config.assets);

        for d in data {
            self.asset_cache.insert(d.id.clone(), d);
        }
    }

    pub fn load(&mut self) {
        self.load_cells();
        self.load_assets();
    }
}
