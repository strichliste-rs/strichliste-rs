use std::rc::Rc;

use leptos::prelude::*;

use crate::{
    backend::core::{behaviour::article_get_articles_for_users::get_articles_per_user, Article},
    frontend::{component::article::search_article::ArticleSearch, model::money_args::MoneyArgs},
    routes::user::components::buy_article::buy_article,
};

#[component]
pub fn BuyArticle(args: Rc<MoneyArgs>) -> impl IntoView {
    let m_clone = args.clone();
    let MoneyArgs {
        user_id,
        money,
        error,
        transactions,
        audio_ref,
    } = *args;
    let personal_articles = OnceResource::new(get_articles_per_user(user_id));
    view! {
        <div>
            <Suspense fallback=move || {
                view! { <p class="text-center text-white">"Loading Articles"</p> }
            }>
                <div class="grid grid-cols-3 text-white text-center gap-2 text-[1.25em] p-2 pt-4">
                    {move || {
                        personal_articles
                            .get()
                            .map(|article| {
                                let article = match article {
                                    Ok(value) => {
                                        value
                                            .into_iter()
                                            .take(9)
                                            .collect::<Vec<Article>>()
                                    }
                                    Err(e) => {
                                        let msg = match e {
                                            ServerFnError::ServerError(msg) => msg,
                                            _ => e.to_string(),
                                        };
                                        return view! {
                                            <p class="bg-red-400 text-white text-center">
                                                {format!("Failed to fetch articles: {}", msg)}
                                            </p>
                                        }
                                            .into_any();
                                    }
                                };
                                article
                                    .into_iter()
                                    .map(|article| {
                                        let Article { id, name, cost, sounds: _, barcodes: _ } = article;

                                        view! {
                                            <button
                                                class="bg-gray-700 rounded p-2"
                                                on:click=move |_| {
                                                    buy_article(
                                                        user_id,
                                                        id,
                                                        money,
                                                        error,
                                                        transactions,
                                                        audio_ref,
                                                    );
                                                }
                                            >
                                                <div>{name}" | "{cost.format_eur()}</div>
                                            </button>
                                        }
                                    })
                                    .collect_view()
                                    .into_any()
                            })
                    }}
                </div>
            </Suspense>
            <ArticleSearch money_args=m_clone.clone() />
        </div>
    }
}
