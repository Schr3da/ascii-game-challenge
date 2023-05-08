use crate::prelude::*;

pub struct Tile {
    pub cells: Vec<Cell>,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Default for Tile {
    fn default() -> Self {
        let cells = vec![Cell::default()];
        let width = cells.len() as i32;

        Tile {
            cells: vec![Cell::default()],
            x: 0,
            y: 0,
            width,
            height: 1,
        }
    }
}
