use std::rc::Rc;

use chrono::{DateTime, Local, Utc};
use leptos::{leptos_dom::logging::console_log, prelude::*};
use leptos_router::hooks::use_params_map;
use tracing::{debug, error, warn};

use crate::{
    models::{Money, Transaction, TransactionType, User, UserId},
    routes::user::get_user,
};

use crate::routes::user::MoneyArgs;

#[server]
pub async fn get_user_transactions(
    user_id: UserId,
    limit: i64,
    offset: i64,
) -> Result<Vec<Transaction>, ServerFnError> {
    use crate::backend::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;
    let response_opts: ResponseOptions = expect_context();

    let transactions =
        Transaction::get_user_transactions(&*state.db.lock().await, user_id, limit, offset).await;

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
pub async fn undo_transaction(user_id: UserId, transaction_id: i64) -> Result<(), ServerFnError> {
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

    // let transaction = Transaction::get(&mut *db_trns, transaction_id, user_id).await;

    // if transaction.is_err() {
    //     error!(
    //         "Failed to fetch transaction: {}",
    //         transaction.err().unwrap()
    //     );
    //     response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
    //     return Err(ServerFnError::new("Failed to fetch transaction!"));
    // }

    // let transaction = transaction.unwrap();
    // if transaction.is_none() {
    //     warn!("A transaction with id '{}' does not exist!", transaction_id);
    //     response_opts.set_status(StatusCode::BAD_REQUEST);
    //     return Err(ServerFnError::new("Invalid transaction!"));
    // }

    // let mut transaction = transaction.unwrap();

    // if transaction.is_undone {
    //     warn!("Attempting to undo a transaction that is already undone!");
    //     response_opts.set_status(StatusCode::BAD_REQUEST);
    //     return Err(ServerFnError::new("The transaction is already undone!"));
    // }

    // match transaction.t_type {
    //     TransactionType::DEPOSIT | TransactionType::WITHDRAW | TransactionType::BOUGHT(_) => {
    //         let new_value = user.money.value - transaction.money.value;

    //         match user.set_money(&mut *db_trns, new_value).await {
    //             Ok(_) => {}
    //             Err(e) => {
    //                 response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
    //                 error!("Failed to update money for user: {}", e);
    //                 return Err(ServerFnError::new("Failed to update user!"));
    //             }
    //         }
    //     }

    //     TransactionType::RECEIVED(recv_from_user_id) => {
    //         let mut recv_from_user = match User::get(&mut *db_trns, recv_from_user_id).await {
    //             Ok(value) => match value {
    //                 Some(value) => value,
    //                 None => {
    //                     error!(
    //                         "Got user db '{}' from database, but the user was not found!",
    //                         recv_from_user_id
    //                     );
    //                     response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
    //                     return Err(ServerFnError::new("Failed to get user!"));
    //                 }
    //             },
    //             Err(e) => {
    //                 error!("Failed to get user to undo transaction: {}", e);
    //                 response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
    //                 return Err(ServerFnError::new("Failed to get user!"));
    //             }
    //         };

    //         match recv_from_user
    //             .add_money(&mut *db_trns, transaction.money.clone())
    //             .await
    //         {
    //             Ok(_) => {}
    //             Err(e) => {
    //                 error!("Failed to upate user money: {}", e);
    //                 response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
    //                 return Err(ServerFnError::new("Failed to update user!"));
    //             }
    //         };

    //         match user
    //             .add_money(&mut *db_trns, (-transaction.money.value).into())
    //             .await
    //         {
    //             Ok(_) => {}
    //             Err(e) => {
    //                 error!("Failed to upate user money: {}", e);
    //                 response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
    //                 return Err(ServerFnError::new("Failed to update user!"));
    //             }
    //         };
    //     }

    //     TransactionType::SENT(sent_to_user) => {
    //         let mut sent_to_user = match User::get(&mut *db_trns, sent_to_user).await {
    //             Ok(value) => match value {
    //                 Some(value) => value,
    //                 None => {
    //                     error!(
    //                         "Got user db '{}' from database, but the user was not found!",
    //                         sent_to_user
    //                     );
    //                     response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
    //                     return Err(ServerFnError::new("Failed to get user!"));
    //                 }
    //             },
    //             Err(e) => {
    //                 error!("Failed to get user to undo transaction: {}", e);
    //                 response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
    //                 return Err(ServerFnError::new("Failed to get user!"));
    //             }
    //         };

    //         match sent_to_user
    //             .add_money(&mut *db_trns, (-transaction.money.value).into())
    //             .await
    //         {
    //             Ok(_) => {}
    //             Err(e) => {
    //                 error!("Failed to upate user money: {}", e);
    //                 response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
    //                 return Err(ServerFnError::new("Failed to update user!"));
    //             }
    //         };

    //         match user
    //             .add_money(&mut *db_trns, transaction.money.clone())
    //             .await
    //         {
    //             Ok(_) => {}
    //             Err(e) => {
    //                 error!("Failed to upate user money: {}", e);
    //                 response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
    //                 return Err(ServerFnError::new("Failed to update user!"));
    //             }
    //         };
    //     }
    // }

    // match transaction.set_undone(&mut *db_trns, true).await {
    //     Ok(_) => {}
    //     Err(e) => {
    //         error!("Failed to set transaction to undone: {}", e);
    //         response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
    //         return Err(ServerFnError::new("Failed to update transaction!"));
    //     }
    // }

    // match db_trns.commit().await {
    //     Ok(_) => {}
    //     Err(e) => {
    //         response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
    //         error!("Failed to apply transaction: {}", e);
    //         return Err(ServerFnError::new("Failed to apply transaction!"));
    //     }
    // }

    // Ok(())
    Err(ServerFnError::new("WIP"))
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

    let user_id = UserId(user_id.unwrap());

    let transaction_data = OnceResource::new(get_user_transactions(user_id, 10, 0));

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
                            key=|transaction| (transaction.id, transaction.is_undone_signal.get(), transaction.timestamp)
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
    user_id: UserId,
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
                TransactionType::Deposit | TransactionType::Withdraw => view!{
                    <p class=""
                        class=("text-green-500", transaction.money.value >= 0)
                        class=("text-red-400", transaction.money.value < 0)
                    >{transaction.money.format_eur_diff()}</p>
                    <p></p>

                }.into_any(),

                TransactionType::Bought(_) => {
                    view!{
                        <p class="text-red-400">{transaction.money.format_eur_diff()}</p>
                        <p class="text-white"><ArticleBasketIcon class="inline"/>" "{transaction.description.clone().unwrap_or("".to_string())}</p>
                    }.into_any()
                },

                TransactionType::SentAndReceived(received_group) => {
                    todo!()
                }

                TransactionType::Received(user)
                | TransactionType::Sent(user) => {
                    let transaction = transaction.clone();
                    view!{}.into_any()
                    // let user = OnceResource::new(get_user(user.clone()));
                    // view!{
                    //     {move || user.get().map(|user| match user {
                    //         Err(e) => {
                    //             let msg = e.to_string();

                    //             view! {
                    //                 <p class=""
                    //                     class=("text-green-500", transaction.money.value >= 0)
                    //                     class=("text-red-400", transaction.money.value < 0)
                    //                 >{transaction.money.format_eur_diff()}</p>
                    //                 <p class="bg-red-400 text-white">"Failed to fetch user: "{msg}</p>
                    //             }.into_any()
                    //         },

                    //         Ok(user) => {
                    //             let user = user.unwrap();
                    //             view! {
                    //                 <p class=""
                    //                     class=("text-green-500", transaction.money.value >= 0)
                    //                     class=("text-red-400", transaction.money.value < 0)
                    //                 >{transaction.money.format_eur_diff()}</p>
                    //                 <p class="text-white flex items-center">
                    //                     {
                    //                         match transaction.money.value > 0 {
                    //                             true => view!{
                    //                                 // received money
                    //                                 <RightArrowIcon class="w-[2rem]"/>{user.nickname}
                    //                             }.into_any(),

                    //                             false => view!{
                    //                                 // sent money
                    //                                 {user.nickname}<RightArrowIcon class="w-[2rem]"/>
                    //                             }.into_any()
                    //                         }
                    //                     }
                    //                 </p>
                    //             }.into_any()
                    //         },
                    //     })}
                    // }.into_any()
                },
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
                                <input type="hidden" name="user_id" value={user_id.0}/>
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

#[component]
pub fn ArticleBasketIcon(class: &'static str) -> impl IntoView {
    view! {
        <div class=class>

            <svg class="inline" height="20px" width="20px" version="1.1" id="_x32_" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512" xml:space="preserve" fill="#000000">

            <g id="SVGRepo_bgCarrier" stroke-width="0"/>

            <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"/>

            <g id="SVGRepo_iconCarrier"> <style type="text/css"> </style> <g> <path class="st0" d="M512,182.161c0-12.088-4.164-23.909-11.996-33.389c-9.964-12.046-24.792-19.027-40.42-19.027H349.003 c-2.382-8.597-7.88-15.895-15.245-20.56l-0.133-66.82l-0.017-0.124c-0.283-13.546-7.797-25.892-19.71-32.323 c-5.582-3.016-11.763-4.532-17.895-4.532c-6.697,0-13.429,1.832-19.377,5.423l-0.016-0.025l-65.146,37.538l-0.216,0.15 c-15.696,9.78-25.725,26.492-27.041,44.919l-0.033,0.624v35.764c-20.844,0.1-40.904,7.864-56.366,21.826l-108.732,98.21 C6.732,260.969,0,276.639,0,292.726c0,5.839,0.883,11.763,2.732,17.511L54.499,472.9c6.381,20.077,25.008,33.714,46.085,33.714 h230.092c25.208,0,49.45-9.706,67.711-27.083l66.995-63.813c8.714-8.314,14.628-19.11,16.911-30.939l0.066-0.383l28.841-193.054 h-0.033C511.701,188.3,512,185.227,512,182.161z M218.996,95.539c0.6-7.164,4.515-13.628,10.597-17.477l64.696-37.288l0.266-0.159 c0.45-0.275,0.916-0.425,1.449-0.425c0.45,0,0.883,0.101,1.316,0.351h0.017c0.883,0.483,1.433,1.399,1.466,2.365l0.149,64.404 c-9.014,4.44-15.861,12.571-18.577,22.435h-36.105v34.813h215.313c2.632,0,5.198,0.592,7.514,1.683l-93.636,86.863 c-9.964,9.03-22.959,14.012-36.388,14.012h-92.07c-2.749-14.778-12.696-26.991-26.075-32.93L218.996,95.539z M151.134,177.438 c9.064-8.188,20.826-12.721,33.022-12.862l-0.033,68.902c-14.245,5.616-24.925,18.244-27.791,33.639H51.85L151.134,177.438z M48.901,340.56l-13.013-40.87c-0.666-2.15-0.999-4.298-1.016-6.464h64.629l5.998,47.334H48.901z M55.832,362.311h52.417 l5.348,42.378H69.328L55.832,362.311z M100.584,471.809c-5.898,0-11.13-3.84-12.912-9.456l-11.43-35.888h40.104l5.732,45.344 H100.584z M188.922,471.809h-44.918l-5.732-45.344h50.65V471.809z M188.922,404.689h-53.399l-5.348-42.378h58.747V404.689z M188.922,340.56h-61.497l-5.998-47.334h67.494V340.56z M198.802,277.28c-6.615,0-11.98-5.381-11.98-11.971 c0-6.623,5.365-11.971,11.98-11.971c6.597,0,11.962,5.348,11.962,11.971C210.765,271.899,205.4,277.28,198.802,277.28z M265.564,471.809h-54.882v-45.344h56.015L265.564,471.809z M267.246,404.689h-56.564v-42.378h57.631L267.246,404.689z M268.846,340.56h-58.164v-47.334h59.364L268.846,340.56z M336.541,471.517c-1.949,0.176-3.916,0.292-5.864,0.292h-43.352 l1.133-45.344h50.666L336.541,471.517z M340.373,404.689h-51.367l1.066-42.378h52.733L340.373,404.689z M344.055,340.56h-53.432 l1.182-47.334h45.27c3.282,0,6.514-0.276,9.747-0.658L344.055,340.56z M399.288,430.598l-24.909,23.716 c-3.416,3.25-7.198,6.041-11.196,8.44l2.449-42.52l36.538-29.357L399.288,430.598z M404.336,361.22l-37.005,29.732l2.315-40.445 l37.655-30.274L404.336,361.22z M409.451,290.593l-38.122,30.64l2.1-36.738c6.298-3.191,12.212-7.19,17.528-11.996l21.243-19.71 L409.451,290.593z M448.055,378.322c-0.917,4.657-3.249,8.906-6.682,12.204l-18.66,17.744l2.616-36.022l26.874-21.592 L448.055,378.322z M456.935,318.966l-29.44,23.643l2.966-40.995l33.022-26.516L456.935,318.966z M468.214,243.366l-35.588,28.616 l2.966-40.886l40.004-37.122L468.214,243.366z"/> </g> </g>

            </svg>
        </div>
    }
}

#[component]
pub fn RightArrowIcon(class: &'static str) -> impl IntoView {
    view! {
        <div class=class>
            <svg viewBox="0 0 24.000001 24.000001" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" fill="#000000"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <filter id="a" color-interpolation-filters="sRGB" height="1.798065" width="1.31919" x="-.159595" y="-.399032"> <feGaussianBlur stdDeviation="1.3302515"></feGaussianBlur> </filter> <g transform="matrix(.90944794 0 0 .90944794 -259.0175 -817.2446)"> <path d="m300.84375 908.875c-.35929.0633-.67275.33918-.78125.6875l-.625 1.8125h-10.4375c-.52358.00005-.99995.47642-1 1v1c.00005.52358.47642.99995 1 1h10.4375l.625 1.8125c.17584.53611.8642.83335 1.375.59375l6-3c.36721-.17625.60257-.59466.5625-1 .001-.0312.001-.0625 0-.0937-.0597-.31022-.27572-.58621-.5625-.71875l-6-3c-.1822-.0907-.39248-.12385-.59375-.0937z" filter="url(#a)" opacity=".2"></path> <path d="m308 911.67377a1.0001 1.0001 0 0 0 -.5625-.71875l-6-3a1.0001 1.0001 0 0 0 -1.375.59375l-.625 1.8125h-10.4375a1.0001 1.0001 0 0 0 -1 1v1a1.0001 1.0001 0 0 0 1 1h10.4375l.625 1.8125a1.0001 1.0001 0 0 0 1.375.59375l6-3a1.0001 1.0001 0 0 0 .5625-1 1.0001 1.0001 0 0 0 0-.0937z"></path> <path d="m307 911.86127-6-3-.84375 2.5h-11.15625v1h11.15625l.84375 2.5z" fill="#fefefe"></path> </g> </g></svg>
        </div>
    }
}

#[component]
pub fn LeftArrowIcon(class: &'static str) -> impl IntoView {
    view! {
        <div class=class>
            <svg viewBox="0 0 24.000001 24.000001" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" fill="#000000" transform="matrix(-1, 0, 0, -1, 0, 0)"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <filter id="a" color-interpolation-filters="sRGB" height="1.798065" width="1.31919" x="-.159595" y="-.399032"> <feGaussianBlur stdDeviation="1.3302515"></feGaussianBlur> </filter> <g transform="matrix(.90944794 0 0 .90944794 -259.0175 -817.2446)"> <path d="m300.84375 908.875c-.35929.0633-.67275.33918-.78125.6875l-.625 1.8125h-10.4375c-.52358.00005-.99995.47642-1 1v1c.00005.52358.47642.99995 1 1h10.4375l.625 1.8125c.17584.53611.8642.83335 1.375.59375l6-3c.36721-.17625.60257-.59466.5625-1 .001-.0312.001-.0625 0-.0937-.0597-.31022-.27572-.58621-.5625-.71875l-6-3c-.1822-.0907-.39248-.12385-.59375-.0937z" filter="url(#a)" opacity=".2"></path> <path d="m308 911.67377a1.0001 1.0001 0 0 0 -.5625-.71875l-6-3a1.0001 1.0001 0 0 0 -1.375.59375l-.625 1.8125h-10.4375a1.0001 1.0001 0 0 0 -1 1v1a1.0001 1.0001 0 0 0 1 1h10.4375l.625 1.8125a1.0001 1.0001 0 0 0 1.375.59375l6-3a1.0001 1.0001 0 0 0 .5625-1 1.0001 1.0001 0 0 0 0-.0937z"></path> <path d="m307 911.86127-6-3-.84375 2.5h-11.15625v1h11.15625l.84375 2.5z" fill="#fefefe"></path> </g> </g></svg>
        </div>
    }
}
