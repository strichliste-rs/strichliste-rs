use std::rc::Rc;

use chrono::{DateTime, Local, Utc};
use leptos::{leptos_dom::logging::console_log, prelude::*};
use leptos_router::hooks::use_params_map;
use tracing::{debug, error, info, warn};

use crate::{
    models::{Money, Transaction, TransactionType, User},
    routes::{articles::get_article, user::get_user},
};

#[cfg(feature = "ssr")]
use crate::models::TransactionDB;

use crate::routes::user::MoneyArgs;

#[server]
pub async fn get_user_transactions(
    user_id: i64,
    limit: i64,
) -> Result<Vec<Transaction>, ServerFnError> {
    use crate::backend::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;

    let response_opts: ResponseOptions = expect_context();

    let transactions =
        Transaction::get_user_transactions(&*state.db.lock().await, user_id, limit).await;

    if transactions.is_err() {
        error!(
            "Failed to fetch transactions: {}",
            transactions.err().unwrap()
        );
        response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Err(ServerFnError::new("Failed to fetch transactions!"));
    }

    let transactions = transactions.unwrap();

    Ok(transactions)
}

#[server]
pub async fn undo_transaction(user_id: i64, transaction_id: i64) -> Result<(), ServerFnError> {
    use crate::backend::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;

    let response_opts: ResponseOptions = expect_context();

    debug!(
        "Need to undo transaction {} for user {}",
        transaction_id, user_id
    );
    let user = get_user(user_id).await?;
    if user.is_none() {
        warn!("A user with id '{}' does not exist!", user_id);
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new("Invalid user!"));
    }
    let mut user = user.unwrap();

    let db = state.db.lock().await;

    let mut db_trns = match db.get_conn_transaction().await {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to get connection to db: {}", e);
            return Err(ServerFnError::new(
                "Failed to create connection to database!",
            ));
        }
    };

    let transaction = Transaction::get(&mut *db_trns, transaction_id).await;

    if transaction.is_err() {
        error!(
            "Failed to fetch transaction: {}",
            transaction.err().unwrap()
        );
        response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Err(ServerFnError::new("Failed to fetch transaction!"));
    }

    let transaction = transaction.unwrap();
    if transaction.is_none() {
        warn!("A transaction with id '{}' does not exist!", transaction_id);
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new("Invalid transaction!"));
    }

    let mut transaction = transaction.unwrap();

    if transaction.is_undone {
        warn!("Attempting to undo a transaction that is already undone!");
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new("The transaction is already undone!"));
    }

    let new_value = user.money.value - transaction.money.value;

    match user.set_money(&mut *db_trns, new_value).await {
        Ok(_) => {}
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to update money for user: {}", e);
            return Err(ServerFnError::new("Failed to update user!"));
        }
    }

    match transaction.set_undone(&mut *db_trns, true).await {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to set transaction to undone: {}", e);
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to update transaction!"));
        }
    }

    match db_trns.commit().await {
        Ok(_) => {}
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to apply transaction: {}", e);
            return Err(ServerFnError::new("Failed to apply transaction!"));
        }
    }

    Ok(())
}

#[component]
pub fn ShowTransactions(arguments: Rc<MoneyArgs>) -> impl IntoView {
    let params = use_params_map();
    let user_id_string = params.read_untracked().get("id").unwrap_or_default();

    let user_id = user_id_string.parse::<i64>();

    if user_id.is_err() {
        return view! {
            <p class="text-red-500">"Failed to convert id to a number!"</p>
        }
        .into_any();
    }

    let user_id = user_id.unwrap();

    let transaction_data = OnceResource::new(get_user_transactions(user_id, 10));

    let transaction_signal: RwSignal<Vec<Transaction>> = arguments.transactions;

    let error_signal = arguments.error;
    let money_signal = arguments.money;

    return view! {
        <Suspense
            fallback=move || view!{<p class="text-white text-center p-5">"Loading transactions"</p>}
        >
        {
            move || {
                let transactions = transaction_data.get();

                if transactions.is_none() {
                    return view!{
                        <p class="text-white bg-red-400 text-center">"Failed to fetch transactions"</p>
                    }.into_any();
                }

                let transactions = transactions.unwrap();

                if transactions.is_err() {
                    let msg = match transactions.err().unwrap() {
                        ServerFnError::ServerError(msg) => msg,
                        _ => "Failed to fetch transactions".to_string()
                    };

                    return view! {
                        <p class="text-white text-center bg-red-400">"Failed to fetch users because: "{msg}</p>
                    }.into_any();
                }

                let mut transactions = transactions.unwrap();
                transactions.sort_by(|a, b| {
                    b.timestamp.cmp(&a.timestamp)
                });
                transaction_signal.write_untracked().append(&mut transactions.into_iter().map(|e| -> Transaction { e.into()}).collect::<Vec<Transaction>>());
                return view! {
                    <div class="pl-4 text-[1.25em]">
                        <For
                            each=move || transaction_signal.get()
                            key=|transaction| (transaction.id, transaction.is_undone_signal.get())
                            let(child)
                        >
                            {format_transaction(&child, user_id, error_signal, money_signal)}
                        </For>

                        // {
                        //      transaction_signal.get().iter().map(|transaction| {
                        //         format_transaction(transaction, user_id, error_write)
                        //     }).collect_view()           
                        // }
                    </div>
                }
                .into_any();
            }
        }

        </Suspense>
    }
    .into_any();
}

pub fn format_transaction(
    transaction: &Transaction,
    user_id: i64,
    error_write: RwSignal<String>,
    money_signal: RwSignal<Money>,
) -> impl IntoView {
    // <svg width="50px" height="50px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path opacity="0.5" d="M4 11.25C3.58579 11.25 3.25 11.5858 3.25 12C3.25 12.4142 3.58579 12.75 4 12.75V11.25ZM4 12.75H20V11.25H4V12.75Z" fill="#a5a4a8" style="--darkreader-inline-fill: var(--darkreader-background-a5a4a8, #161f3d);" data-darkreader-inline-fill=""></path> <path d="M14 6L20 12L14 18" stroke="#a5a4a8" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="--darkreader-inline-stroke: var(--darkreader-text-a5a4a8, #acc4e0);" data-darkreader-inline-stroke=""></path> </g></svg>
    let now: DateTime<Utc> = Utc::now();
    let diff = now - transaction.timestamp;

    let undo_action = ServerAction::<UndoTransaction>::new();
    let transaction_id = transaction.id;

    let date_string = format!(
        "{}",
        transaction
            .timestamp
            .with_timezone(&Local)
            .format("%d.%m.%Y %H:%M:%S")
    );

    let undo_signal = transaction.is_undone_signal;

    let money = transaction.money.value;

    return view! {
        <div class="grid grid-cols-3 items-center border-t-4 border-gray-300 p-2 text-white"
            class=("line-through", undo_signal.get())
        >
        {
            match transaction.t_type {
                TransactionType::DEPOSIT | TransactionType::WITHDRAW => view!{
                    <p class=""
                        class=("text-green-500", transaction.money.value >= 0)
                        class=("text-red-400", transaction.money.value < 0)
                    >{transaction.money.format_eur_diff()}</p>
                    <p></p>

                }.into_any(),

                TransactionType::BOUGTH(_) => {
                    view!{
                        <p class="text-red-400">{transaction.money.format_eur_diff()}</p>
                        <p class="text-white">{transaction.description.clone().unwrap_or("".to_string())}</p>
                    }.into_any()
                },

                _ => view!{}.into_any(),
            }
        }
        {
            move || match undo_signal.get() {
                true => {
                    // console_log("Re-rendering date");
                    view!{
                        <p class="text-white">{date_string.clone()}</p>
                    }.into_any()
                },
                false => {

                    // grace period for undoing transactions
                    // if transaction is already undone, only show the date regardless of grace period
                    if diff.num_minutes() > 2 {
                        view!{
                            <p class="text-white">{date_string.clone()}</p>
                        }.into_any()
                    } else {
                        view! {
                            <ActionForm action=undo_action>
                                <input type="hidden" name="user_id" value={user_id}/>
                                <input type="hidden" name="transaction_id" value={transaction_id}/>
                                <input type="submit" class="text-white" value="Undo"/>
                            </ActionForm>
                        }.into_any()
                    }
                }
            }
        }
        {

            move || match undo_action.value().get() {
                    None => {},
                    Some(response) => {
                        match response {
                            Ok(_) => {
                                undo_signal.set(true);
                                money_signal.update(|value| (*value).value = value.value - money);
                                console_log("Set signal to true");
                                error_write.set(String::new());
                            },
                            Err(e) => {
                                let msg = match e {
                                    ServerFnError::ServerError(msg) => msg,
                                    _ => e.to_string(),
                                };

                                error_write.set(msg);
                            }
                        }
                    }
                }
        }
        </div>
    };
}
