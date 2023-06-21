use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Tsify)]
pub enum CommandPopupIds {
    UnknownCommandPopupId,
}

impl Default for CommandPopupIds {
    fn default() -> Self {
        CommandPopupIds::UnknownCommandPopupId
    }
}
