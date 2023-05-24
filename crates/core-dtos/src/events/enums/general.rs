#[derive(Clone)]
pub enum GeneralEvents {
    OnApplicationWillInitialise,
    OnApplicationWillClose,
}

#[derive(Clone, Eq, PartialEq)]
pub enum GeneralSubscription {
    OnApplicationDidInitialise,
    OnApplicationDidClose,
}
