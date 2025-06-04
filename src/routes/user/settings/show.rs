use leptos::{prelude::*, server_fn::codec::IntoRes};
use leptos_router::hooks::use_params_map;
use tracing::{debug, error, warn};

use crate::{models::User, routes::user::get_user};

#[server] pub async fn update_user(id: i64, nickname: String, card_number: String,) -> Result<(), ServerFnError> {
    use crate::backend::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;
    use leptos_axum::redirect;

    let response_opts: ResponseOptions = expect_context();

    let user = get_user(id).await;

    if user.is_err(){
        error!("Failed to fetch user: {}", user.err().unwrap());
        response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Err(ServerFnError::new("Failed to fetch user!"));
    }

    let user = user.unwrap();

    if user.is_none() {
        warn!("No such user with id '{}' exists!", id);
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new("No such user exists!"));
    }

    let mut user = user.unwrap();

    let card_number_exists = User::get_by_card_number(&*state.db.lock().await, &card_number).await;

    if card_number_exists.is_err() {
        error!("Failed to check for existence of the card number: {}", card_number_exists.err().unwrap());
        response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Err(ServerFnError::new("Failed to check if the card number is already used!"));
    }

    let card_number_exists = card_number_exists.unwrap();

    if card_number_exists.is_some() {
        let user = card_number_exists.unwrap();
        if user.id.unwrap() != id {
            warn!("The card number '{}' is already used!", card_number);
            response_opts.set_status(StatusCode::BAD_REQUEST);
            return Err(ServerFnError::new("The card number is already used!"));   
        }
    }

    user.nickname = nickname;
    user.card_number = match card_number.len() {
        0 => None,
        _ => Some(card_number)
    };

    debug!("Changing card number for user '{}' to '{:?}'", user.id.unwrap(), user.card_number);

    let result = user.update_db(&*state.db.lock().await).await;

    if result.is_err() {
        error!("Failed to update user in db: {}", result.err().unwrap());
        response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Err(ServerFnError::new("Failed to update user!"));
    }

    redirect(&format!("/user/{}", id));
    
    Ok(())
}

#[component]
pub fn Show() -> impl IntoView {
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

    let update_action = ServerAction::<UpdateUser>::new();
    return view! {
        <Suspense
            fallback=move ||view!{<p class="text-white text-center pt-5">"Loading User..."</p>}
        >
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

                return view!{
                    {
                        move || match update_action.value().get() {
                             Some(Err(e)) => {
                                let msg = match e {
                                    ServerFnError::ServerError(msg) => msg,
                                    _ => e.to_string(),
                                };

                                
                                return view! {<p class="p-3 bg-red-400 text-white text-center">"Failed to update user: "{msg}</p>}.into_any();
                            },

                             _ => {
                                 view!{}.into_any()
                             },
                        }
                    }
                    <ActionForm action=update_action>
                        <div class="flex flex-col items-center gap-5">
                            <div class="flex flex-col items-center">
                                <label class="text-white text-[1.25em]">"Nickname"</label>
                                <input class="text-[1.25em]" type="text" value={user.nickname} name="nickname"/>
                            </div>
                            <div class="flex flex-col items-center">
                                <label class="text-white text-[1.25em]">"Card number"</label>
                                <input class="text-[1.25em]" type="text" value={user.card_number} name="card_number"/>
                            </div>
                            <input type="hidden" value={user.id.unwrap()} name="id"/>
                            <input class="text-white hover:bg-pink-700 bg-emerald-700 rounded-full text-[1.25em] p-2" type="submit" value="Update user"/>
                        </div>
                        </ActionForm>
                }.into_any();

            }
        }
        </Suspense>
    
    }
    .into_any();
}
