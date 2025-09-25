use crate::model::AudioPlayback;
use reactive_stores::Store;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Store)]
pub struct FrontendStore {
    // #[store(key: i64 = |user| user.id.unwrap())]
    pub cached_sounds: HashMap<AudioPlayback, String>,
}
