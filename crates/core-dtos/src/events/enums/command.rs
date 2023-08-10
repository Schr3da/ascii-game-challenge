use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, Deserialize, Serialize, Tsify)]
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
