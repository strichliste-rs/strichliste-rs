use leptos::prelude::NodeRef;
use reactive_stores::Store;
use std::collections::HashMap;

use crate::frontend::model::caching_layer::CachingLayerType;

pub type FrontendStoreType = Store<FrontendStore>;

#[derive(Clone, Debug, Store)]
pub struct FrontendStore {
    pub cached_sounds: HashMap<String, String>,
    pub audio_ref: NodeRef<leptos::html::Audio>,
    pub cachinglayer: CachingLayerType,
}
