use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Clone, Deserialize, Serialize, Tsify)]
pub enum CommandInputEvents {
    New,
    Pop,
    Push(String),
    Execute(Vec<String>),
    Cancel,
}

#[derive(Debug, Clone, Eq, PartialEq, Tsify, Serialize, Deserialize)]
pub enum CommandSubscription {
    OnCommandDidUpdate(Vec<String>),
}
