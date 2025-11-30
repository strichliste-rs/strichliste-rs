pub const PREFIX_FILTER_NON_ALPHABETIC_VALUE: char = '!';

use itertools::Itertools;
use leptos::prelude::*;

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

    let users = Signal::derive(move || {
        cached_users
            .read()
            .value
            .get()
            .into_iter()
            .filter(|elem| {
                elem.nickname
                    .to_lowercase()
                    .starts_with(&user_filter.get().to_lowercase())
            })
            .collect_vec()
    });

    view! {
        <HomeHeader user_filter />
        <UserList users />
    }
}
