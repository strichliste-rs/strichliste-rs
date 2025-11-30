pub const PREFIX_FILTER_NON_ALPHABETIC_VALUE: char = '!';

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

    let users = CachingLayer::get_all_users(cache);

    view! {
        <HomeHeader />
        <UserList users />
    }
}
