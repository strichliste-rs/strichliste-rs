use leptos::{html, prelude::*, task::spawn_local};
use leptos_router::hooks::use_params_map;
use tracing::error;

use crate::{
    models::{Money, Transaction, User},
    routes::{home::get_all_users, user::get_user},
};

#[server]
pub async fn send_money(
    from_user: i64,
    to_user: String,
    amount: String,
) -> Result<(), ServerFnError> {
    use crate::backend::ServerState;
    let state: ServerState = expect_context();

    use axum::http::StatusCode;
    use leptos_axum::redirect;
    use leptos_axum::ResponseOptions;

    let response_opts: ResponseOptions = expect_context();

    let money: Money = match amount.clone().try_into() {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::BAD_REQUEST);
            return Err(ServerFnError::new(&format!(
                "Failed to convert '{}' to internal representation: {}",
                amount, e
            )));
        }
    };

    if money.value < 0 {
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new("Amount to be sent must be > 0!"));
    }

    let mut from_user = match get_user(from_user).await? {
        Some(value) => value,
        None => {
            response_opts.set_status(StatusCode::BAD_REQUEST);
            return Err(ServerFnError::new(
                "The user you are trying to send the money from does not exist!",
            ));
        }
    };

    let db = state.db.lock().await;

    let mut db_trans = match db.get_conn_transaction().await {
        Ok(value) => value,
        Err(e) => {
            error!("Failed to get db transaction: {}", e);
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to acquire db transaction!"));
        }
    };

    let mut to_user = match User::get_by_nick(&mut *db_trans, to_user.clone()).await {
        Ok(value) => match value {
            Some(value) => value,
            None => {
                response_opts.set_status(StatusCode::BAD_REQUEST);
                return Err(ServerFnError::new(&format!(
                    "There is no such user with the nick '{}'!",
                    to_user
                )));
            }
        },

        Err(e) => {
            error!("Failed to get user by nick: {}", e);
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to get user by nick!"));
        }
    };

    if from_user.id == to_user.id {
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new(
            "Sending and receiving user must not be the same!",
        ));
    }

    match from_user
        .add_money(&mut *db_trans, (-money.value).into())
        .await
    {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to apply new money value: {}", e);
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to apply money!"));
        }
    }

    match to_user.add_money(&mut *db_trans, money.clone()).await {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to apply new money value: {}", e);
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to apply money!"));
        }
    }

    _ = match Transaction::create(
        &mut *db_trans,
        from_user.id,
        crate::models::TransactionType::SENT(to_user.id),
        None,
        (-money.value).into(),
    )
    .await
    {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to create transaction: {}", e);
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to create transaction!"));
        }
    };

    _ = match Transaction::create(
        &mut *db_trans,
        to_user.id,
        crate::models::TransactionType::RECEIVED(from_user.id),
        None,
        money.clone(),
    )
    .await
    {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to create transaction: {}", e);
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to create transaction!"));
        }
    };

    match db_trans.commit().await {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to commit transaction: {}", e);
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to apply transaction!"));
        }
    };

    redirect(&format!("/user/{}", from_user.id));

    Ok(())
}

#[component]
pub fn Show() -> impl IntoView {
    let params = use_params_map();
    let user_id_string = params.read_untracked().get("id").unwrap_or_default();

    let user_id = match user_id_string.parse::<i64>() {
        Ok(value) => value,
        Err(_e) => {
            return view! {
                <p class="text-red-500">"Failed to convert id to a number!"</p>
            }
            .into_any();
        }
    };

    let user_resource = OnceResource::new(get_user(user_id));
    let all_users_resource = OnceResource::new(get_all_users());

    let receiver_input = RwSignal::new(String::new());
    let amount_input = RwSignal::new(String::new());
    let hidden_div_node_ref = NodeRef::<html::Div>::new();

    let error_result = RwSignal::new(String::new());
    view! {
        <Suspense
            fallback=move || view!{<p class="text-white text-center">"Loading user"</p>}
        >
            {move || user_resource.get().map(|user| {
                let user = match user {
                    Ok(value) => value,
                    Err(e) => {
                        let e = e.to_string();
                        return view!{
                            <p class="bg-red-400 text-white text-center">"Failed to fetch user: "{e}</p>
                        }.into_any();
                    }
                };

                let user = match user {
                    Some(value) => value,
                    None => {
                        return view!{
                            <p class="bg-red-400 text-white text-center">"No such user with id '"{user_id}"' exists!"</p>
                        }.into_any();
                    }
                };

                let all_users = match all_users_resource.get() {
                    Some(Ok(value)) => value,
                    _ => return view!{
                        <p class="bg-red-400 text-white text-center">"Failed to fetch all users!"</p>
                    }.into_any(),
                };

                return view!{
                <div class="flex h-screen bg-gray-900">
                    <div class="w-full max-w-xs m-auto bg-indigo-100 rounded p-5">
                        <p class="text-indigo-500 text-center">"Hello "{user.nickname}", who do you want to send money to?"</p>
                        <div>
                          <label class="block mb-2 text-indigo-500">"Receiver"</label>
                          <input bind:value=receiver_input autocomplete="off" class="w-full p-2 mb-6 text-indigo-700 border-b-2 border-indigo-500 outline-none focus:bg-gray-300" type="text" name="to_user"/>
                        </div>
                        <div class="flex flex-col items-center"
                            class=("hidden", move || receiver_input.get().len() == 0)
                            node_ref=hidden_div_node_ref
                        >
                        {move || {
                            let search = receiver_input.get();

                            all_users.iter().filter(|elem| elem.id != user.id).filter(|elem| elem.nickname.to_lowercase().contains(&search.to_lowercase())).map(|elem| {
                                let nickname = elem.nickname.clone();
                                let n_clone = nickname.clone();
                                view!{
                                    <button class="bg-gray-400 text-white p-2 rounded"
                                        on:click=move |_| {
                                            receiver_input.set(n_clone.clone());
                                            hidden_div_node_ref.get().map(|elem| elem.class("hidden flex flex-col items-center")); // just hidden would be nice, but .class nukes all other classes
                                        }
                                    >
                                        {nickname}
                                    </button>
                                }
                            }).collect_view()
                        }}
                        </div>
                        <div>
                          <label class="block mb-2 text-indigo-500">"Amount"</label>
                          <input bind:value=amount_input autocomplete="off" class="w-full p-2 mb-6 text-indigo-700 border-b-2 border-indigo-500 outline-none focus:bg-gray-300" type="text" name="amount"/>
                        </div>
                        <div>
                            <input class="w-full bg-indigo-700 hover:bg-pink-700 text-white font-bold py-2 px-4 mb-6 rounded" type="submit" value="Send money"
                                on:click=move |_| {
                                    spawn_local(async move {
                                        match send_money(user_id, receiver_input.get(), amount_input.get()).await {
                                            Ok(_) => {},
                                            Err(e) => {
                                                error_result.set(e.to_string());
                                            }
                                        }
                                    });
                                }
                            />
                        </div>
                        <div>
                            {move || match error_result.get().len() {
                                0 => view! {}.into_any(),
                                _ => {
                                    let msg = error_result.get();
                                    view! { <p class="text-red-900">"Failed to send money:  "{msg}</p>}.into_any()
                                },
                            }}
                        </div>
                    </div>
                </div>
            }.into_any();
        })}
        </Suspense>
    }.into_any()
}
