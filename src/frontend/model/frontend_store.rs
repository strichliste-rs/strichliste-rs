use leptos::prelude::{NodeRef, RwSignal};
use reactive_stores::Store;
use std::collections::HashMap;

use crate::{backend::core::User, frontend::model::cachinglayer::CachingLayerType};

pub type FrontendStoreType = Store<FrontendStore>;

#[derive(Clone, Debug, Store)]
pub struct FrontendStore {
    pub cached_sounds: HashMap<String, String>,
    pub audio_ref: NodeRef<leptos::html::Audio>,
    pub error: RwSignal<Vec<String>>,
    pub cached_users: Vec<User>,
    pub cachinglayer: CachingLayerType,
}
