use leptos::{html::Audio, prelude::*};
use leptos_router::hooks::use_params_map;

use crate::models::Article;

#[server]
pub async fn get_article(article_id: i64) -> Result<Article, ServerFnError> {
    use crate::backend::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;

    let response_opts: ResponseOptions = expect_context();

    let article = Article::get_from_db(&*state.db.lock().await, article_id).await;

    let article = match article {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new(format!(
                "Error getting article from db: {}",
                e
            )));
        }
    };

    match article {
        None => {
            response_opts.set_status(StatusCode::BAD_REQUEST);
            Err(ServerFnError::new(format!(
                "Unknown Article id '{}'",
                article_id
            )))
        }

        Some(value) => Ok(value),
    }
}

#[component]
pub fn Show() -> impl IntoView {
    let params = use_params_map();
    let article_id_string = params
        .read_untracked()
        .get("article_id")
        .unwrap_or_default();

    let article_id = article_id_string.parse::<i64>();
    let article_id = match article_id {
        Ok(value) => value,
        Err(err) => {
            return view! {
                <p class="text-red-400 text-center">"Failed to get id from params map: "{err.to_string()}</p>
            }
            .into_any();
        }
    };

    let article_resource = OnceResource::new(get_article(article_id));

    view! {
        {move || match article_resource.get() {
            None => {
                return view! {
                    <p class="text-white text-center">"Loading article..."</p>
                }.into_any();
            },

            Some(value) => {
                let article = match value {
                  Ok(value) => value,
                  Err(e) => {
                      let error_msg = match e {
                          ServerFnError::ServerError(msg) => msg,
                          _ => e.to_string(),
                      };
                      return view!{
                          <p class="text-red-400 text-center">"Failed to load Article: "{error_msg}</p>
                      }.into_any();
                  }
                };

                return view!{
                    <SingleArticleView article/>
                }.into_any()
            }
        }}
    }
    .into_any()
}

#[component]
fn SingleArticleView(article: Article) -> impl IntoView {
    view! {}
}
