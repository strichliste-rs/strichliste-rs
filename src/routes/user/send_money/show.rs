use leptos::prelude::*;

use crate::models::Money;

#[server]
pub async fn send_money(from_user: i64, to_user: i64, amount: Money) -> Result<(), ServerFnError> {
    Ok(())
}

#[component]
pub fn Show() -> impl IntoView {
    let send_money_action = ServerAction::<SendMoney>::new();
    view! {
        <p class="text-white text-center">"Sending money!"</p>
        <div class="flex h-screen bg-gray-900">
            <div class="w-full max-w-xs m-auto bg-indigo-100 rounded p-5">
              <ActionForm action=send_money_action>
                <div>
                  <label class="block mb-2 text-indigo-500" for="username">Nickname</label>
                  <input class="w-full p-2 mb-6 text-indigo-700 border-b-2 border-indigo-500 outline-none focus:bg-gray-300" type="text" name="username"/>
                </div>
                <div>
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
    }
}
