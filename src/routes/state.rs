use std::collections::HashMap;

use reactive_stores::Store;

use crate::model::AudioPlayback;

#[derive(Clone, Debug, Default, Store)]
pub struct FrontendStore {
    // #[store(key: i64 = |user| user.id.unwrap())]
    pub cached_sounds: HashMap<AudioPlayback, String>,
}
