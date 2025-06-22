use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::{
    models::Money,
    routes::{home::get_all_users, user::get_user},
};

#[server]
pub async fn send_money(
    from_user: i64,
    to_user: String,
    amount: String,
) -> Result<(), ServerFnError> {
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

    let send_money_action = ServerAction::<SendMoney>::new();
    let user_resource = OnceResource::new(get_user(user_id));
    let all_users_resource = OnceResource::new(get_all_users());

    let receiver_input = RwSignal::new(String::new());
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
                      <ActionForm action=send_money_action>
                        <p class="text-indigo-500 text-center">"Hello "{user.nickname}", who do you want to send money to?"</p>
                        <div>
                          <label class="block mb-2 text-indigo-500">"Receiver"</label>
                          <input bind:value=receiver_input class="w-full p-2 mb-6 text-indigo-700 border-b-2 border-indigo-500 outline-none focus:bg-gray-300" type="text" name="to_user"/>
                        </div>
                        <div class="flex flex-col items-center"
                            class=("hidden", receiver_input.get().len() == 0)
                        >
                        {move || {
                            let search = receiver_input.get();

                            all_users.iter().filter(|elem| elem.id != user.id).filter(|elem| elem.nickname.to_lowercase().contains(&search.to_lowercase())).map(|elem| {
                                let nickname = elem.nickname.clone();
                                let n_clone = nickname.clone();
                                view!{
                                    <button
                                        on:click=move |_| {
                                            receiver_input.set(n_clone.clone());
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
                          <input class="w-full p-2 mb-6 text-indigo-700 border-b-2 border-indigo-500 outline-none focus:bg-gray-300" type="text" name="amount"/>
                        </div>
                        <div>
                            <input type="hidden" name="from_user" value=user.id/>
                          <input class="w-full bg-indigo-700 hover:bg-pink-700 text-white font-bold py-2 px-4 mb-6 rounded" type="submit" value="Create Account"/>
                        </div>
                      </ActionForm>
                        <div>
                            {move || match send_money_action.value().get() {
                                Some(Err(e)) => {
                                    let msg = match e {
                                        ServerFnError::ServerError(msg) => msg,
                                        _ => e.to_string(),
                                    };

                                    view! { <p class="text-red-900">"Failed to create user: "{msg}</p>}.into_any()
                                },
                                _ => view! {}.into_any(),
                            }}
                        </div>
                    </div>
                </div>
            }.into_any();
        })}
        </Suspense>
    }.into_any()
}
