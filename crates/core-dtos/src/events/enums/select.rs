use tsify::Tsify;

#[derive(Clone, Tsify)]
pub enum SelectionDirections {
    Next,
    Previous,
}
