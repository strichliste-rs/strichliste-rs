use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use reactive_stores::{Store, StoreField};
use tracing::error;

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

    let user = User::get_by_id(&*state.db.lock().await, id).await;

    if user.is_err() {
        let err = user.err().unwrap();
        error!("Failed to fetch user: {}", err);
        response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Err(ServerFnError::new(err));
    }

    let user = user.unwrap();

    Ok(user)
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

    let user_resource = OnceResource::new(get_user(user_id));

    return view! {
        { move || {

            view!{
                <Suspense
                    fallback=move || view!{<p class="text-white text-center pt-5">"Loading user..."</p>}
                >
                <div>
                    {
                        move || {
                            let user = user_resource.get();

                            if user.is_none() {
                                return view!{
                                    <p class="text-red-500">"Failed to fetch user"</p>
                                }.into_any();
                            }

                            let user = user.unwrap();

                            if user.is_err(){
                                let err = user.err().unwrap().to_string();
                                return view!{
                                    <p class="text-red-500">"Failed to fetch user because: "{err}</p>
                                }.into_any();
                            }

                            let user = user.unwrap();

                            if user.is_none(){
                                return view! {
                                    <p class="text-red-500">"No user with the id "{user_id}" has been found!"</p>
                                }.into_any();
                            }

                            let user = user.unwrap();

                            view!{
                                <div class="grid grid-cols-2">
                                    <div class="pt-5">
                                        // left side
                                        <p class="text-center text-white text-lg">{user.nickname.clone()}</p>
                                        <p class="text-center text-lg"
                                            class=("text-red-500", move || user.money < 0)
                                            class=("text-green-500", move ||user.money >= 0)

                                        >{user.get_money()}"€"</p>
                                    </div>
                                    <div>
                                        // right side
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }
                </div>
                </Suspense>
            }
        }.into_any()
        }
    }
    .into_any();
}
