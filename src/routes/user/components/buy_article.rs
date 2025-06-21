use std::rc::Rc;

use leptos::{ev, html, leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use tracing::error;

use crate::{
    models::{Article, Money, Transaction},
    routes::{
        articles::{get_all_articles, get_article},
        user::{get_user, MoneyArgs},
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
    let articles_resource = OnceResource::new(get_all_articles(None));
    let MoneyArgs {
        user_id,
        money,
        error,
        transactions,
    } = *args;
    let article_signal = RwSignal::new(Vec::<Article>::new());
    view! {
        <div>
            <Suspense
            fallback=move || view!{<p class="text-center text-white">"Loading Articles"</p>}
            >
            <div class="grid grid-cols-3 text-white text-center gap-2 text-[1.25em] p-2 pt-4">
            {
                move || {
                    articles_resource.get().map(|article| {
                        let article = match article {
                            Ok(value) => {
                                article_signal.update(|val| *val = value.clone());
                                value.into_iter().take(9).collect::<Vec<Article>>()
                            },
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
                            let Article { id, name, cost, sounds: _, barcodes: _ } = article;
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
            <ArticleSearch articles=article_signal user_id=user_id money=money error=error transactions=transactions/>
        </div>
    }
}

#[component]
pub fn ArticleSearch(
    articles: RwSignal<Vec<Article>>,
    user_id: i64,
    money: RwSignal<Money>,
    error: RwSignal<String>,
    transactions: RwSignal<Vec<Transaction>>,
) -> impl IntoView {
    let dropdown_div = NodeRef::<html::Div>::new();
    let search_term = RwSignal::new(String::new());
    let filtered_articles = RwSignal::new(Vec::<Article>::new());
    let on_input = move |_ev: ev::Event| {
        match search_term.get().len() {
            0 => filtered_articles.set(Vec::<Article>::new()),
            _ => filtered_articles.update(|val| {
                *val = articles
                    .get()
                    .iter()
                    .filter(|elem| elem.name.contains(&search_term.get()))
                    .take(5)
                    .map(|elem| elem.clone())
                    .collect::<Vec<Article>>();
            }),
        };
    };

    view! {
        <div class="w-full min-w-[200px] flex flex-col items-center p-2">
            <div class="relative">
                <input
                  class="text-white w-full bg-transparent placeholder:text-slate-400 text-slate-700 text-sm border border-slate-200 rounded-md pl-3 pr-28 py-2 transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-300 shadow-sm focus:shadow"
                  placeholder="Search for articles"
                  autocomplete=false
                  bind:value=search_term
                  on:input=on_input
                />
                <button
                  class="absolute top-1 right-1 flex items-center rounded bg-slate-800 py-1 px-2.5 border border-transparent text-center text-sm text-white transition-all shadow-sm hover:shadow focus:bg-slate-700 focus:shadow-none active:bg-slate-700 hover:bg-slate-700 active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none"
                  type="button"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-4 h-4 mr-2">
                    <path fill-rule="evenodd" d="M10.5 3.75a6.75 6.75 0 1 0 0 13.5 6.75 6.75 0 0 0 0-13.5ZM2.25 10.5a8.25 8.25 0 1 1 14.59 5.28l4.69 4.69a.75.75 0 1 1-1.06 1.06l-4.69-4.69A8.25 8.25 0 0 1 2.25 10.5Z" clip-rule="evenodd" />
                  </svg>

                  Search
                </button>
            </div>
            <div
                node_ref=dropdown_div
                class=("hidden", move || search_term.get().len() == 0)
            >
            {move || {
                filtered_articles.get().into_iter().map(|elem| {
                    view!{
                        <button
                            on:click=move |_| {
                                buy_article(user_id, elem.id, money, error, transactions);
                                search_term.set(String::new());
                            }
                        >
                            <div class="p-2 m-2 rounded text-white bg-gray-700">
                                <p>{elem.name.clone()}" | "{elem.cost.format_eur()}</p>
                            </div>
                        </button>
                    }.into_any()
                }).collect_view()
            }}
            </div>
        </div>
    }.into_any()
}
