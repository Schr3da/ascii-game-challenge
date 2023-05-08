use tui::widgets::Block;

pub trait Drawable {
    fn draw(&self) -> Block;
}
