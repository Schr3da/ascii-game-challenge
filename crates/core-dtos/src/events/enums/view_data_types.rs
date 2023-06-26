use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Eq, PartialEq, Clone, Default, Serialize, Deserialize, Tsify)]
pub enum ViewDataTypes {
    #[default]
    NoViewData,
    QuickActionData,
}
