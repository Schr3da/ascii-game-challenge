use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tsify::*;

use crate::prelude::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize, Tsify, Eq, PartialEq)]
pub struct LoggerState {
    #[serde(rename(serialize = "currentLogs", deserialize = "currentLogs"))]
    pub current_logs: Vec<String>,
}

impl From<&LoggerState> for HashMap<ViewComponentIds, String> {
    fn from(value: &LoggerState) -> HashMap<ViewComponentIds, String> {
        let mut next = HashMap::new();
        next.insert(
            ViewComponentIds::Popup(PopupIds::Log(Some(LoggerIds::Print))),
            format!("{:?}", value.current_logs),
        );

        next
    }
}
