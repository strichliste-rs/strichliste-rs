use leptos::prelude::{NodeRef, RwSignal};
use reactive_stores::Store;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Store)]
pub struct FrontendStore {
    pub cached_sounds: HashMap<String, String>,
    pub audio_ref: NodeRef<leptos::html::Audio>,
    pub error: RwSignal<Vec<String>>,
}
