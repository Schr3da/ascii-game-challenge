use std::collections::HashMap;

use core_dtos::prelude::*;

pub struct _SpriteManager {
    pub cache: HashMap<AssetTypes, String>,
}
