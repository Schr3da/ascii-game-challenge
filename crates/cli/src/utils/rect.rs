use tui::layout::Rect;

pub fn rect_center(parent: Rect, width: u16, height: u16) -> Rect {
    let next = Rect {
        x: parent.x + (parent.width - width) / 2,
        y: parent.y + (parent.height - height) / 2,
        width,
        height,
    };

    next
}
