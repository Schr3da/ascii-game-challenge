use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum GeneralEvents {
    OnApplicationResize(u16, u16),
    OnApplicationWillInitialise(u16, u16),
    OnApplicationWillClose,
}

#[derive(Debug, Clone, Eq, PartialEq, Tsify, Serialize, Deserialize)]
pub enum GeneralSubscription {
    OnApplicationDidStart,
    OnApplicationDidLoadAssets(HashMap<AsciiIds, Cell>),
    OnApplicationDidInitialise,
    OnApplicationDidClose,
}
