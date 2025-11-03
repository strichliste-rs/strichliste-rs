use leptos::{
    prelude::{Effect, GetUntracked, ReadSignal, ReadUntracked, RwSignal, Set, Write},
    reactive::spawn_local,
};

use crate::{
    backend::core::{behaviour::user_get::get_user, User},
    frontend::model::cachinglayer::{CachingEntry, CachingLayer, CachingLayerType},
    model::UserId,
};

impl CachingLayer {
    fn fetch_user(user_id: UserId, entry: RwSignal<CachingEntry<Option<User>>>) {
        Effect::new(move || {
            spawn_local(async move {
                entry.write().is_fetching.set(true);
                match get_user(user_id).await {
                    Ok(value) => {
                        entry.write().is_fetching.set(false);
                        entry.write().value.set(value);
                    }
                    Err(e) => {
                        entry.write().is_fetching.set(false);
                        entry
                            .write()
                            .error
                            .set(Some(format!("Failed to load user: {e}")));
                    }
                }
            })
        });
    }
    pub fn get_user(
        layer: CachingLayerType,
        user_id: UserId,
    ) -> ReadSignal<CachingEntry<Option<User>>> {
        let users = layer.read_untracked().cached_users.read_untracked();

        let user = users
            .value
            .get_untracked()
            .into_iter()
            .find(|val| val.id == user_id);

        let cache_entry = CachingEntry::default();

        cache_entry.value.set(user);

        let cache_entry = RwSignal::new(cache_entry);

        CachingLayer::fetch_user(user_id, cache_entry);

        cache_entry.read_only()
    }
}
