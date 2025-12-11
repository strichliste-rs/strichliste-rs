pub const PREFIX_FILTER_NON_ALPHABETIC_VALUE: char = '!';

use itertools::Itertools;
use leptos::prelude::*;
use thaw::Spinner;

use crate::frontend::{
    component::home::{HomeHeader, UserList},
    model::{
        caching_layer::CachingLayer,
        frontend_store::{FrontendStoreStoreFields, FrontendStoreType},
    },
};

#[component]
pub fn View() -> impl IntoView {
    let store = expect_context::<FrontendStoreType>();
    let cache = store.cachinglayer().get_untracked();

    let cached_users = CachingLayer::get_all_users(cache);

    let user_filter = RwSignal::new(String::new());

    view! {
        <HomeHeader user_filter />

        {move || {
            let (is_fetching, value) = {
                let entry = cached_users.read();
                (entry.is_fetching, entry.value)
            };
            if is_fetching.get() && value.read().is_empty() {
                return view! { <Spinner label="Loading users!" /> }.into_any();
            }

            let users = Signal::derive(move || value.read().iter().filter(|user| user.nickname.to_lowercase().starts_with(&user_filter.get())).cloned().collect_vec());

            view!{
                <UserList users />
            }.into_any()
        }}
    }
}
