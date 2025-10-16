use leptos::prelude::*;
use thaw::{Button, ToasterInjection};

use crate::{
    backend::core::{behaviour::article_get_articles_for_users::get_articles_per_user, Article},
    frontend::{
        component::article::search_article::ArticleSearch,
        model::money_args::MoneyArgs,
        shared::{buy_article, throw_error_none_view},
    },
};

#[component]
pub fn BuyArticle(args: RwSignal<MoneyArgs>) -> impl IntoView {
    let toaster = ToasterInjection::expect_context();

    let personal_articles = OnceResource::new(get_articles_per_user(args.get_untracked().user_id));
    view! {
        <div>
            <Suspense fallback=move || {
                view! { <p class="text-center text-white">"Loading Articles"</p> }
            }>
                <div class="grid grid-cols-3 text-white text-center gap-2 text-[1.25em] p-2 pt-4">
                    {move || {
                        personal_articles
                            .get()
                            .map(move |article| {
                                let article = match article {
                                    Ok(value) => value.into_iter().take(9).collect::<Vec<Article>>(),
                                    Err(e) => {
                                        let msg = match e {
                                            ServerFnError::ServerError(msg) => msg,
                                            _ => e.to_string(),
                                        };
                                        return throw_error_none_view(
                                            format!("Failed to fetch articles: {}", msg),
                                        );
                                    }
                                };
                                article
                                    .into_iter()
                                    .filter(|article| !article.is_disabled)
                                    .map(|article| {
                                        let Article { id, name, cost, .. } = article;
                                        view! {
                                            <Button
                                                class="bg-gray-700 rounded p-2"
                                                on_click=move |_| {
                                                    buy_article(id, cost, args, toaster);
                                                }
                                            >
                                                <div>{name}" | "{cost.format_eur()}</div>
                                            </Button>
                                        }
                                    })
                                    .collect_view()
                                    .into_any()
                            })
                    }}
                </div>
            </Suspense>
            <ArticleSearch money_args=args />
        </div>
    }
}
