use leptos::{
    html::{self, Audio},
    leptos_dom::logging::console_log,
    prelude::*,
    task::spawn_local,
};
use leptos_router::hooks::use_params_map;

use crate::models::{Article, Barcode};

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

/// TODO: Fix client side error
#[server]
pub async fn update_article(article: Article) -> Result<(), ServerFnError> {
    Ok(())
}

#[component]
pub fn Edit() -> impl IntoView {
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
    let name_node = NodeRef::<html::Input>::new();
    let cost_node = NodeRef::<html::Input>::new();
    let barcode_signal = RwSignal::new(Vec::<Barcode>::new());
    let class_css = "flex flex-col gap-5";
    let input_css = "ml-5 text-black rounded-[5px] text-center";

    let clone = article.clone();

    let on_click = move |_| {
        let mut article = clone.clone();
        article.name = name_node.get().unwrap().value();
        console_log("Hello");

        spawn_local(async move {
            let result = update_article(article).await;
        });
    };
    return view! {
        <div class="flex flex-col items-center pt-5 gap-10">
            <div class="flex justify-center pt-5 text-[1.25em]">
                <div class=format!("{} items-end", {class_css})>
                    <a class="text-white">"Name:"</a>
                    <a class="text-white">"Cost:"</a>

                </div>
                <div class=format!("{} items-center", {class_css})>
                    <input class={input_css} type="text" value={article.name} node_ref=name_node/>
                    <input class={input_css} type="text" value={article.cost.value} node_ref=cost_node/>
                </div>
            </div>

            <div class="w-[30vw]">
                <input class="w-full bg-indigo-700 hover:bg-pink-700 text-white font-bold py-2 px-4 mb-6 rounded" type="submit" value="Upate article" on:click=on_click/>
            </div>

        </div>
    }
    .into_any();
}
