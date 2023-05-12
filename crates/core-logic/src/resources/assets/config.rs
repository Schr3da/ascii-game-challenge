use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AssetConfig {
    pub cells: String,
    pub assets: String,
}
