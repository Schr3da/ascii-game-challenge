use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::{ToSelectable, ViewComponentIds};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Tsify)]
pub enum CommandIds {
    Move,
    Build,
    Inspect,
    UnknownCommandId,
}

impl Default for CommandIds {
    fn default() -> Self {
        CommandIds::Move
    }
}

impl ToSelectable for CommandIds {
    type Item = ViewComponentIds;

    fn get_selectable_items() -> Vec<ViewComponentIds> {
        vec![
            ViewComponentIds::CommandPopup(CommandIds::Move),
            ViewComponentIds::CommandPopup(CommandIds::Build),
            ViewComponentIds::CommandPopup(CommandIds::Inspect),
        ]
    }
}
