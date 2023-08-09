use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Clone, Tsify, Serialize, Deserialize)]
pub enum Keys {
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    Enter,
    Esc,
    Backspace,
    Tab,
    BackTab,
    Char(char),
}

#[derive(Clone, Tsify, Serialize, Deserialize)]
pub enum MouseEvent {
    OnPress(bool),
    OnMove(i32, i32),
}

#[derive(Clone, Tsify, Serialize, Deserialize)]
pub enum KeyboardEvent {
    OnPress(Keys),
}

pub enum InputEvents {
    Keyboard(KeyboardEvent),
    Mouse(MouseEvent),
}
