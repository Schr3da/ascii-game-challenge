#[derive(Clone)]
pub enum GeneralEvents {
    OnApplicationResize(u16, u16),
    OnApplicationWillInitialise,
    OnApplicationWillClose,
}

#[derive(Clone, Eq, PartialEq)]
pub enum GeneralSubscription {
    OnApplicationDidInitialise,
    OnApplicationDidClose,
}
