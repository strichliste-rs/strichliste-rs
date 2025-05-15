use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use reactive_stores::{Store, StoreField};

use crate::{
    models::User,
    routes::state::{FrontendStore, FrontendStoreStoreFields},
};

#[server]
pub async fn get_user(id: i64) -> Result<Option<User>, ServerFnError> {
    use crate::backend::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;

    let response_opts: ResponseOptions = expect_context();

    Ok(None)
}

#[component]
pub fn ShowUser() -> impl IntoView {
    let params = use_params_map();
    let user_id_string = params.read_untracked().get("id").unwrap_or_default();

    let user_id = user_id_string.parse::<i64>();

    if user_id.is_err() {
        return view! {
            <p class="text-red-500">"Failed to convert id to a number!"</p>
        }
        .into_any();
    }

    let user_id = user_id.unwrap();

    let store = expect_context::<Store<FrontendStore>>();

    return view! {
        { move || {
            let cached_users = store.cached_users();
            let reader = cached_users.reader().unwrap();
            let mut found_user: Option<&User> = None;
            for user in reader.iter() {
                if user.id.unwrap() == user_id {
                    found_user = Some(user);
                }
            }

            if found_user.is_none() {
                return view!{
                    <p class="text-red-500">"Failed to find user with id "{user_id}"!"</p>
                }.into_any();
            }

            let user: &User = found_user.unwrap();


            view!{
                <div>
                    <p class="text-center">{user.nickname.clone()}</p>
                    <p class="text-center">{user.money / 100}"€"</p>
                </div>
            }
        }.into_any()
        }
    }
    .into_any();
}
