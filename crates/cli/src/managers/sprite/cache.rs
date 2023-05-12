use std::collections::HashMap;

use core_logic::prelude::AssetTypes;

pub struct _SpriteManager {
  pub cache: HashMap<AssetTypes, String>
}