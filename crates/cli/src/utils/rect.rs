use std::cmp::max;
use tui::layout::Rect;

pub fn rect_center(parent: Rect, width: u16, height: u16) -> Rect {
    let next = Rect {
        x: 0,
        y: 0,
        width,
        height,
    };

    next
}
