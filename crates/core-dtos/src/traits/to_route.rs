use crate::prelude::*;

pub trait ToRoute {
    fn to_view_route(&self) -> Option<UiViewIds>;

    fn to_popup_route(&self) -> Option<UiPopupViewIds>;
}
