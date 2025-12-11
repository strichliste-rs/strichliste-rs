use leptos::{
    prelude::{Effect, ReadSignal, ReadUntracked, Set, Write},
    reactive::spawn_local,
};

use crate::{
    backend::core::{behaviour::user_get_all::get_all_users, User},
    frontend::model::{
        caching_entry::CachingEntry,
        caching_layer::{CachingLayer, CachingLayerType},
    },
};

impl CachingLayer {
    fn fetch_all_users(layer: CachingLayerType) {
        layer
            .write_only()
            .write()
            .cached_users
            .write()
            .is_fetching
            .set(true);
        Effect::new(move || {
            spawn_local(async move {
                let write_cached = layer.write_only();

                let write_cached = write_cached.write();

                match get_all_users().await {
                    Ok(value) => {
                        write_cached.cached_users.write().value.set(value);
                        write_cached.cached_users.write().is_fetching.set(false);
                    }
                    Err(e) => write_cached
                        .cached_users
                        .write()
                        .error
                        .set(Some(format!("Failed to fetch users: {e}"))),
                };
            })
        });
    }
    pub fn get_all_users(layer: CachingLayerType) -> ReadSignal<CachingEntry<Vec<User>>> {
        CachingLayer::fetch_all_users(layer);
        layer.read_only().read_untracked().cached_users.read_only()
    }
}
