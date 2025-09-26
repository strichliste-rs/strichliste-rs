use std::rc::Rc;

use leptos::{leptos_dom::logging::console_log, prelude::*};
use leptos_router::hooks::use_params_map;
use leptos_use::{use_infinite_scroll_with_options, UseInfiniteScrollOptions};

use crate::{
    backend::core::behaviour::transaction_get_user_transactions::get_user_transactions,
    frontend::{component::transaction::format_transaction, model::money_args::MoneyArgs},
    model::{PageRequestParams, PageResponseParams, Transaction, UserId},
};

#[component]
pub fn ShowTransactions(arguments: Rc<MoneyArgs>) -> impl IntoView {
    let params = use_params_map();
    let user_id_string = match params.read_untracked().get("id") {
        Some(s) => s,
        None => {
            return view! { <p class="text-red-500">"Failed to obtain id from url"</p> }.into_any();
        }
    };

    let user_id = match user_id_string.parse::<i64>() {
        Ok(user_id) => UserId(user_id),
        Err(_) => {
            return view! { <p class="text-red-500">"Failed to convert id to a number!"</p> }
                .into_any();
        }
    };

    let previous_transactions_presonse_params: RwSignal<Option<PageResponseParams>> =
        RwSignal::new(None);
    let transaction_data =
        OnceResource::new(get_user_transactions(user_id, PageRequestParams::new(100)));

    let transaction_signal = arguments.transactions;
    let money_signal = arguments.money;

    view! {
        <Suspense fallback=move || {
            view! { <p class="text-white text-center p-5">"Loading transactions"</p> }
        }>
            {move || {
                let transactions = match transaction_data.get() {
                    Some(transactions) => transactions,
                    None => {
                        return view! {
                            <p class="text-white bg-red-400 text-center">
                                "Failed to fetch transactions"
                            </p>
                        }
                            .into_any();
                    }
                };
                let mut transactions = match transactions {
                    Ok(transactions) => transactions.items,
                    Err(err) => {
                        let msg = match err {
                            ServerFnError::ServerError(msg) => msg,
                            _ => "Failed to fetch transactions".to_string(),
                        };
                        return view! {
                            <p class="text-white text-center bg-red-400">
                                "Failed to fetch users because: "{msg}
                            </p>
                        }
                            .into_any();
                    }
                };
                transactions.sort_by(|a, b| { b.timestamp.cmp(&a.timestamp) });
                let el = NodeRef::<leptos::html::Div>::new();
                transaction_signal
                    .write()
                    .append(&mut transactions.into_iter().collect::<Vec<Transaction>>());
                Effect::new(move |_| {
                    let _ = use_infinite_scroll_with_options(
                        el,
                        move |_| async move {
                            let next_params = previous_transactions_presonse_params
                                .with_untracked(|p| PageResponseParams::next_params(*p, 100));
                            if let Some(params) = next_params {
                                let mut data = get_user_transactions(user_id, params).await;
                                match data {
                                    Ok(mut data) => {
                                        transaction_signal.update(|d| d.append(&mut data.items));
                                        previous_transactions_presonse_params
                                            .set(Some(data.params));
                                    }
                                    Err(e) => console_log(&e.to_string()),
                                }
                            }
                        },
                        UseInfiniteScrollOptions::default().distance(20.0).interval(1.0),
                    );
                });

                view! {
                    <div class="pl-4 text-[1.25em] h-[800px] w-full overflow-y-scroll" node_ref=el>
                        <For
                            each=move || transaction_signal.get()
                            key=|transaction| (
                                transaction.id,
                                transaction.is_undone_signal.get(),
                                transaction.timestamp,
                            )
                            let:child
                        >
                            {format_transaction(&child, user_id, money_signal)}
                        </For>

                    </div>
                }
                    .into_any()
            }}

        </Suspense>
    }
    .into_any()
}
