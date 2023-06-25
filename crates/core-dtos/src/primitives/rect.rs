use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Tsify)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn set_top(&mut self, next: u16) {
        self.y = next as i32;
    }

    pub fn set_width(&mut self, next: u16) {
        self.width = next as i32;
    }

    pub fn set_height(&mut self, next: u16) {
        self.height = next as i32;
    }
}
