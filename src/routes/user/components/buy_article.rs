use std::rc::Rc;

use leptos::{leptos_dom::logging::console_log, prelude::*};

use crate::{
    models::{Article, Money, Transaction},
    routes::{articles::get_all_articles, user::MoneyArgs},
};

pub fn buy_article(
    user_id: i64,
    article_id: i64,
    money: RwSignal<Money>,
    error: RwSignal<String>,
    transactions: RwSignal<Vec<Transaction>>,
) {
    console_log(&format!("Need to buy article with id: {}", article_id))
}

#[component]
pub fn BuyArticle(args: Rc<MoneyArgs>) -> impl IntoView {
    let articles_resource = OnceResource::new(get_all_articles());
    let MoneyArgs {
        user_id,
        money,
        error,
        transactions,
    } = *args;
    view! {
        <Suspense
        fallback=move || view!{<p class="text-center text-white">"Loading Articles"</p>}
        >
        <div class="grid grid-cols-3 text-white text-center gap-2 text-[1.25em] pt-2">
        {
            move || {
                articles_resource.get().map(|article| {
                    let article = match article {
                        Ok(value) => value,
                        Err(e) => {
                            let msg = match e {
                              ServerFnError::ServerError(msg) => msg,
                              _ => e.to_string(),
                            };
                            return view!{
                                <p class="bg-red-400 text-white text-center">{format!("Failed to fetch articles: {}", msg)}</p>
                            }.into_any();
                        }
                    };

                    article.into_iter().map(|article| {
                        let Article { id, name, cost, sounds, barcodes } = article;
                        view!{
                                <button class="bg-gray-700 rounded p-2"
                                    on:click=move |_| {
                                        buy_article(user_id, id.unwrap(), money, error, transactions);
                                    }
                                >
                                    <div>
                                        {name}" | "{cost.format_eur()}
                                    </div>
                                </button>
                        }
                    }).collect_view().into_any()

                })
            }
        }
        </div>
        </Suspense>
    }
}
