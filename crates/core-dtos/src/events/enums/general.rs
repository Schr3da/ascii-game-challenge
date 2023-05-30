use tsify::Tsify;

#[derive(Clone, Tsify)]
pub enum GeneralEvents {
    OnApplicationResize(u16, u16),
    OnApplicationWillInitialise(u16, u16),
    OnApplicationWillClose,
}

#[derive(Clone, Eq, PartialEq, Tsify)]
pub enum GeneralSubscription {
    OnApplicationDidInitialise,
    OnApplicationDidClose,
}
