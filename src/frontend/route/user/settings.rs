use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::{
    backend::core::behaviour::{update_user::UpdateUser, user_get::get_user},
    frontend::shared::throw_error_none_view,
    model::UserId,
};

#[component]
pub fn Show() -> impl IntoView {
    let params = use_params_map();
    let user_id_string = params.read_untracked().get("id").unwrap_or_default();

    let user_id = match user_id_string.parse::<i64>() {
        Ok(user_id) => UserId(user_id),
        Err(_) => {
            return throw_error_none_view(format!(
                "Failed to convert id: {user_id_string} to a number!"
            ));
        }
    };

    let user_resource = OnceResource::new(get_user(user_id));

    let update_action = ServerAction::<UpdateUser>::new();
    view! {
        <Suspense fallback=move || {
            view! { <p class="text-white text-center pt-5">"Loading User..."</p> }
        }>
            {move || {
                let user = match user_resource.get() {
                    Some(user) => user,
                    None => {
                        return ().into_any();
                    }
                };
                let user = match user {
                    Ok(user) => user,
                    Err(err) => {
                        return throw_error_none_view(
                            format!("Failed to fetch user because: {err}"),
                        );
                    }
                };
                let user = match user {
                    Some(user) => user,
                    None => {
                        return throw_error_none_view(
                            format!("No user with the id {} has been found!", user_id.0),
                        );
                    }
                };

                view! {
                    {move || match update_action.value().get() {
                        Some(Err(e)) => {
                            let msg = match e {
                                ServerFnError::ServerError(msg) => msg,
                                _ => e.to_string(),
                            };
                            throw_error_none_view(format!("Failed to update user: {msg}"))
                        }
                        _ => ().into_any(),
                    }}
                    <ActionForm action=update_action>
                        <div class="flex flex-col items-center gap-5">
                            <div class="flex flex-col items-center">
                                <label class="text-white text-[1.25em]">"Nickname"</label>
                                <input
                                    class="text-[1.25em] text-black"
                                    type="text"
                                    value=user.nickname
                                    name="nickname"
                                />
                            </div>
                            <div class="flex flex-col items-center">
                                <label class="text-white text-[1.25em]">"Card number"</label>
                                <input
                                    class="text-[1.25em] text-black"
                                    type="text"
                                    value=user.card_number
                                    name="card_number"
                                />
                            </div>
                            <input type="hidden" value=user.id.0 name="id" />
                            <input
                                class="text-white hover:bg-pink-700 bg-emerald-700 rounded-full text-[1.25em] p-2"
                                type="submit"
                                value="Update user"
                            />
                        </div>
                    </ActionForm>
                }
                    .into_any()
            }}
        </Suspense>
    }
    .into_any()
}
