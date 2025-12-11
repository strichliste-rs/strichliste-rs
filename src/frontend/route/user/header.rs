use leptos::{
    component,
    prelude::{expect_context, Get, GetUntracked, IntoAny, Read, ReadUntracked, RwSignal},
    view, IntoView,
};
use leptos_router::hooks::use_params_map;
use thaw::Spinner;

use crate::{
    backend::core::User,
    frontend::{
        component::user::header::UserHeader,
        model::{
            caching_layer::CachingLayer,
            frontend_store::{FrontendStoreStoreFields, FrontendStoreType},
        },
        shared::throw_error_none_view,
    },
    model::UserId,
};

#[component]
pub fn header_view() -> impl IntoView {
    let params = use_params_map();
    let user_id_string = params.read_untracked().get("id").unwrap_or_default();

    let user_id = match user_id_string.parse::<i64>() {
        Ok(id) => UserId(id),
        Err(e) => {
            return throw_error_none_view(format!("Failed to convert id to a number: {e}"));
        }
    };

    let store = expect_context::<FrontendStoreType>();

    let cache = store.cachinglayer().get_untracked();

    let entry = CachingLayer::get_user(cache, user_id);

    view! {
        {move || {
            let (is_fetching, value) = {
                let entry = entry.read();
                (entry.is_fetching, entry.value)
            };

            if is_fetching.get() && value.get().is_none() {
                return view! { <Spinner label="Loading user!"/> }.into_any();
            }

            let user: RwSignal<Option<User>> = RwSignal::new(value.get());

            view!{

                // <ReturnTo after=15 route="/"/>
                <UserHeader user/>
            }.into_any()
        }}
    }
    .into_any()
}
