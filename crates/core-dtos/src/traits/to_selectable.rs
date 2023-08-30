pub trait ToSelectable {
    type Item;
    fn get_selectable_items() -> Vec<Self::Item>;
}
