use std::rc::Rc;

use chrono::Utc;
use leptos::{leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use tracing::error;

use crate::{
    models::{Article, Money, Transaction},
    routes::{
        articles::{get_all_articles, get_article},
        user::{create_transaction, get_user, MoneyArgs},
    },
};

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

    let db = state.db.lock().await;
    let mut db_trans = match db.get_conn_transaction().await {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to create db_transaction: {}", e);
            return Err(ServerFnError::new("Failed to create db connection"));
        }
    };

    let transaction = match Transaction::create(
        &mut *db_trans,
        user_id,
        crate::models::TransactionType::BOUGTH(article_id),
        Some(article.name.clone()),
        cost,
    )
    .await
    {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to create transaction: {}", e);
            return Err(ServerFnError::new("Failed to create transaction"));
        }
    };
    let new_value = user.money.value + transaction.money.value;

    match user.set_money(&mut *db_trans, new_value).await {
        Ok(_) => {}
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to update money for user in db: {e}");
            return Err(ServerFnError::new("Failed to update money for user in db!"));
        }
    }

    match db_trans.commit().await {
        Ok(_) => {}
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to commit transaction: {e}");
            return Err(ServerFnError::new("Failed to commit the db transaction!"));
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
                                        buy_article(user_id, id, money, error, transactions);
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
