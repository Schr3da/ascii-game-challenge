#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiViewIds {
    Main,
    Game,
    Options,
}

impl Default for UiViewIds {
    fn default() -> Self {
        UiViewIds::Main
    }
}
