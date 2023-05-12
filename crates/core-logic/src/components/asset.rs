use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum AssetTypes {
    Wall,
    UnknownAssetType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: AssetTypes,
    pub cells: Vec<Cell>,
    pub description: Option<String>,
}

impl Default for Asset {
    fn default() -> Self {
        Asset {
            id: AssetTypes::UnknownAssetType,
            cells: Vec::new(),
            description: None,
        }
    }
}
