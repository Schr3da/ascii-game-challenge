use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Clone, Deserialize, Serialize, Tsify)]
pub enum InputEvents {
  New,
  Pop,
  Push(String),
  Execute(Vec<String>),
  Cancel,
}