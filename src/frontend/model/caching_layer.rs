use leptos::prelude::RwSignal;

use crate::{backend::core::User, frontend::model::caching_entry::CachingEntry};

pub type CachingLayerType = RwSignal<CachingLayer>;

#[derive(Default)]
pub struct CachingLayer {
    pub cached_users: RwSignal<CachingEntry<Vec<User>>>,
}
