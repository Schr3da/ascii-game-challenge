use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::prelude::*;

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum GeneralEvents {
    OnApplicationResize(u16, u16),
    OnApplicationWillInitialise(u16, u16),
    OnApplicationWillClose,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum UiEvents {
    OnSelect(SelectionDirections),
    OnSelectById(ViewComponentIds),
    OnClick(ViewComponentIds),
    OnShortcut(String),
    OnOpenPopup(UiPopupViewIds),
    OnClosePopup,
    OnCloseView,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum RenderEvents {
    OnUpdateCamera(Navigation),
    OnUpdateSelectedCell(Navigation),
    OnWorldWillUpdate,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
pub enum SendEvents {
    General(GeneralEvents),
    Ui(UiEvents),
    Renderer(RenderEvents),
    Tick,
}

#[derive(Debug, Clone, Eq, PartialEq, Tsify, Serialize, Deserialize)]
pub enum GeneralSubscription {
    OnApplicationDidStart,
    OnApplicationDidLoadAssets(HashMap<AsciiIds, Cell>),
    OnApplicationDidInitialise,
    OnApplicationDidClose,
}

#[derive(Debug, Clone, Eq, PartialEq, Tsify, Serialize, Deserialize)]
pub enum UiSubscription {
    UnknownUiSubscription,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Tsify)]
pub enum RenderSubscription {
    OnWorldDidUpdate(Option<UiView>, Option<UiView>, GameMeta),
}

#[derive(Debug, Clone, Eq, PartialEq, Tsify, Serialize, Deserialize)]
pub enum SubscriptionEvents {
    General(GeneralSubscription),
    Ui(UiSubscription),
    Renderer(RenderSubscription),
}

#[derive(Clone, Tsify)]
pub enum EcsEvents {
    Send(SendEvents),
    Subscriber(SubscriptionEvents),
}
