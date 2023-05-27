#[derive(Debug, Clone, PartialEq, Eq)]
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