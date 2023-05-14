use std::collections::HashMap;

use core_ecs::prelude::AssetTypes;

pub struct _SpriteManager {
  pub cache: HashMap<AssetTypes, String>
}