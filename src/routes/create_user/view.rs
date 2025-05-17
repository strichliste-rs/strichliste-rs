use leptos::prelude::*;
use tracing::{debug, error};

use crate::models::User;

#[server]
pub async fn create_user(username: String) -> Result<(), ServerFnError> {
    use crate::backend::ServerState;
    let state: ServerState = expect_context();

    use axum::http::StatusCode;
    use leptos_axum::redirect;
    use leptos_axum::ResponseOptions;

    let response_opts: ResponseOptions = expect_context();
    debug!("Creating account!");

    if username.len() == 0 {
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new("Name cannot be empty!"));
    }
    let mut user = User::new();
    user.nickname = username.trim().to_string();

    let result = user.add_to_db(&*state.db.lock().await).await;

    if result.is_err() {
        response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        let error = result.err().unwrap();
        error!("Failed to add user: {}", error);
        return Err(ServerFnError::new(error));
    }

    redirect(&format!("/user/{}", user.id.unwrap()));

    Ok(())
}

#[component]
pub fn View() -> impl IntoView {
    let create_user_action = ServerAction::<CreateUser>::new();
    view! {
        <div class="flex h-screen bg-gray-900">
            <div class="w-full max-w-xs m-auto bg-indigo-100 rounded p-5">
              <ActionForm action=create_user_action>
                <div>
                  <label class="block mb-2 text-indigo-500" for="username">Nickname</label>
                  <input class="w-full p-2 mb-6 text-indigo-700 border-b-2 border-indigo-500 outline-none focus:bg-gray-300" type="text" name="username"/>
                </div>
                <div>
                  <input class="w-full bg-indigo-700 hover:bg-pink-700 text-white font-bold py-2 px-4 mb-6 rounded" type="submit" value="Create Account"/>
                </div>
              </ActionForm>
                <div>
                    {move || match create_user_action.value().get() {
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
