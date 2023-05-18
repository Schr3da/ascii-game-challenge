#[derive(Clone)]
pub enum GeneralEvents {
    OnApplicationWillInitialise,
    OnApplicationWillClose,
}

#[derive(Clone)]
pub enum GeneralSubscription {
    OnApplicationDidInitialise,
    OnApplicationDidClose,
}
