#[derive(Debug)]
pub enum UiViewIds {
    Main,
    Options,
}

impl Default for UiViewIds {
    fn default() -> Self {
        UiViewIds::Main
    }
}
