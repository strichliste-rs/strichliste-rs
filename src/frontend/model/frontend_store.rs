use crate::model::AudioPlayback;
use leptos::prelude::*;
use reactive_stores::Store;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Store)]
pub struct FrontendStore {
    // #[store(key: i64 = |user| user.id.unwrap())]
    pub cached_sounds: HashMap<AudioPlayback, String>,
    pub audio_ref: NodeRef<leptos::html::Audio>,
    pub error: RwSignal<Vec<String>>,
}
