use crate::prelude::UiViewChild;

pub trait ToUiViewChildren{
    fn get_ui_items() -> Vec<UiViewChild>;
}
