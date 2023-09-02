use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Tsify)]
pub enum TextAlignment {
    Center,
    Left,
    Right,
}

impl Default for TextAlignment {
    fn default() -> Self {
        TextAlignment::Center
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Component, Serialize, Deserialize, Tsify)]
pub struct UiLabel {
    pub id: ViewComponentIds,
    pub text: String,
    pub alignment: TextAlignment,
    pub shortcut: Option<String>,
}

impl UiLabel {
    pub fn format(&self) -> String {
        match &self.shortcut {
            Some(s) => format!("[{}] {}", s, self.text),
            None => format!("{}", self.text),
        }
    }

    pub fn format_with_data(&self, view_data: &ViewDataTypes) -> String {
        let data: HashMap<ViewComponentIds, String> = view_data.into();

        let title = match data.get(&self.id) {
            Some(v) => v,
            None => &self.text,
        };

        match &self.shortcut {
            Some(s) => format!("[{}] {}", s, title),
            None => format!("{}", title),
        }
    }
}
