use leptos::prelude::*;
use tracing::debug;

use crate::models::{Article, ArticleDB, Money};

#[server]
pub async fn create_article(name: String, cost: String) -> Result<(), ServerFnError> {
    use crate::backend::ServerState;
    let state: ServerState = expect_context();

    use axum::http::StatusCode;
    use leptos_axum::redirect;
    use leptos_axum::ResponseOptions;

    let response_opts: ResponseOptions = expect_context();
    debug!("Creating article!");

    if name.len() == 0 {
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new("Name cannot be empty!"));
    }

    if cost.len() == 0 {
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new("Cost cannot be empty!"));
    }

    let money: Money = match cost.try_into() {
        Ok(value) => value,
        Err(err) => {
            response_opts.set_status(StatusCode::BAD_REQUEST);
            return Err(ServerFnError::new(err));
        }
    };

    let article = Article::new(name, money);

    let mut article_db: ArticleDB = article.into();

    match article_db.add_to_db(&*state.db.lock().await).await {
        Ok(()) => {}
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new(e));
        }
    }

    redirect(&format!("/articles/{}", article_db.id.unwrap()));

    Ok(())
}

#[component]
pub fn Create() -> impl IntoView {
    let create_article_action = ServerAction::<CreateArticle>::new();
    view! {

        <div class="flex h-screen bg-gray-900">
            <div class="w-full max-w-xs m-auto bg-indigo-100 rounded p-5">
              <ActionForm action=create_article_action>
                <div>
                  <label class="block mb-2 text-indigo-500">"Name: "</label> <input class="w-full p-2 mb-6 text-indigo-700 border-b-2 border-indigo-500 outline-none focus:bg-gray-300" type="text" name="name"/>
                  <label class="block mb-2 text-indigo-500">"Cost: "</label> <input class="w-full p-2 mb-6 text-indigo-700 border-b-2 border-indigo-500 outline-none focus:bg-gray-300" type="text" name="cost"/>
                </div>
                <div>
                  <input class="w-full bg-indigo-700 hover:bg-pink-700 text-white font-bold py-2 px-4 mb-6 rounded" type="submit" value="Create Article"/>
                </div>
              </ActionForm>
                <div>
                    {move || match create_article_action.value().get() {
                        Some(Err(e)) => {
                            let msg = match e {
                                ServerFnError::ServerError(msg) => msg,
                                _ => e.to_string(),
                            };

                            view! { <p class="text-red-900">"Failed to create article: "{msg}</p>}.into_any()
                        },
                        _ => view! {}.into_any(),
                    }}
                </div>
            </div>
        </div>
    }
}
