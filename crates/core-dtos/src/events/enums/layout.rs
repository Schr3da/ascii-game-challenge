#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LayoutAlignments {
    Horizontal,
    Vertical,
}

impl Default for LayoutAlignments {
    fn default() -> Self {
        LayoutAlignments::Vertical
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LayoutConstraints {
    Percentage(u16),
}

impl Default for LayoutConstraints {
    fn default() -> Self {
        LayoutConstraints::Percentage(100)
    }
}
