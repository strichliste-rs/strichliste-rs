use std::rc::Rc;

use chrono::Utc;
use leptos::{leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use tracing::error;

use crate::{
    models::{Article, Money, Transaction},
    routes::{
        articles::{get_all_articles, get_article},
        user::{get_user, MoneyArgs},
    },
};

use super::transaction_view::create_transaction;

#[server]
pub async fn buy_article_by_id(
    user_id: i64,
    article_id: i64,
) -> Result<Transaction, ServerFnError> {
    use crate::backend::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;

    let response_opts: ResponseOptions = expect_context();

    let article = get_article(article_id).await?;
    let user = get_user(user_id).await?;

    let mut user = match user {
        Some(user) => user,
        None => {
            response_opts.set_status(StatusCode::BAD_REQUEST);
            return Err(ServerFnError::new("Invalid user id given!"));
        }
    };

    // Article costs are positive, but the transaction should subtract money from the user
    let mut cost = article.cost.clone();
    cost.value *= -1;

    let transaction = Transaction {
        id: None,
        user_id,
        is_undone: false,
        t_type: crate::models::TransactionType::BOUGTH(article_id),
        money: cost,
        description: Some(article.name.clone()),
        timestamp: Utc::now(),
        is_undone_signal: RwSignal::new(false),
    };

    let transaction = create_transaction(transaction).await?;

    user.money.value += transaction.money.value;

    match user.update_money(&*state.db.lock().await).await {
        Ok(_) => {}
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to update money for user in db: {e}");
            return Err(ServerFnError::new("Failed to update money for user in db!"));
        }
    }

    Ok(transaction)
}

pub fn buy_article(
    user_id: i64,
    article_id: i64,
    money: RwSignal<Money>,
    error: RwSignal<String>,
    transactions: RwSignal<Vec<Transaction>>,
) {
    console_log(&format!("Need to buy article with id: {}", article_id));
    spawn_local(async move {
        match buy_article_by_id(user_id, article_id).await {
            Ok(transaction) => {
                money.update(|money| money.value += transaction.money.value);
                transactions.update(|trns| trns.insert(0, transaction));
                error.set(String::new());
            }

            Err(e) => {
                error.set(format!("Failed to buy article: {}", e));
            }
        }
    });
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
        <div class="grid grid-cols-3 text-white text-center gap-2 text-[1.25em] p-2 pt-4">
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
