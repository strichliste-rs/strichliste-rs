use leptos::prelude::*;
use tracing::error;

use crate::models::{Article, User};

#[component]
pub fn View() -> impl IntoView {
    view! {
        <div class="grid grid-cols-10 gap-10 py-10">
            <div class="col-span-1 pl-5">
                <a href="/articles/create">
                    <div class="flex justify-center">
                        // joinked from: https://gist.github.com/ibelick/0c92c1aba54c2f7e8b3a7381426ed029
                        <button class="inline-flex h-10 w-10 items-center justify-center rounded-full bg-gray-50 text-black drop-shadow-sm transition-colors duration-150 hover:bg-gray-200">
                            "+"
                        </button>
                    </div>
                </a>
            </div>
            <div class="col-span-9 pr-7">
                <ShowArticles/>
            </div>
        </div>
    }
}

#[component]
fn ShowArticles() -> impl IntoView {
    let all_articles = OnceResource::new(get_all_articles());
    view! {
            <Suspense
                    fallback=move || view!{<h1>"Loading articles..."</h1>}
            >
                {move || all_articles.get().map(|articles| {
                    match articles{
                        Err(err) => {
                            let msg = match err {
                                ServerFnError::ServerError(msg) => msg,
                                _ => err.to_string()
                            };
                            view!{
                                <p class="text-red-900"> "Failed to fetch article: " {msg} </p>
                            }.into_any()
                        },
                        Ok(articles) => view!{
                            <table>
                            <tr>
                              <th>"Name"</th>
                              <th>"Preis"</th>
                            </tr>
                            {
                                articles.into_iter().map(|article| {
                                    view!{
                                        <tr>
                                          <td>{article.name}</td>
                                          <td>{article.cost.format_eur()}</td>
                                        </tr>
                                    }
                                }).collect_view()

                            }
                            </table>
                        }.into_any(),
                    }
                })
                }
            </Suspense>
    }
}

#[server]
pub async fn get_all_articles() -> Result<Vec<Article>, ServerFnError> {
    use crate::backend::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;

    let response_opts: ResponseOptions = expect_context();

    let articles = Article::get_all_from_db(&*state.db.lock().await).await;
    articles.map_err(|e| {
        let err = e.to_string();
        error!("Could not fetch articles {}", err);
        response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        ServerFnError::new(err)
    })
}
