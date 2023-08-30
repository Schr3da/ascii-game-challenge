pub trait Constructable {
    type Args;
    fn new(args: Self::Args) -> Self;
}
