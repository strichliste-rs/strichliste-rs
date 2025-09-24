pub use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::{
    backend::core::behaviour::transaction_get_user_transactions::get_user_transactions,
    model::{Money, PageRequestParams, Transaction, UserId},
    routes::user::{
        components::transaction_view::format_transaction, extra_transactions::ShowNavigationButtons,
    },
};

#[component]
pub fn Show() -> impl IntoView {
    let params = use_params_map();
    let user_id_string = params.read_untracked().get("id").unwrap_or_default();

    let user_id = match user_id_string.parse::<i64>() {
        Ok(value) => UserId(value),
        Err(_) => {
            return view! { <p class="text-red-400">"Invalid user id"</p> }.into_any();
        }
    };

    let transactions_per_page = 15;

    let page_count = RwSignal::new(0);
    let trans_resource = Resource::new(
        move || (page_count.get(), user_id, transactions_per_page),
        |(page_count, user_id, transactions_per_page)| {
            get_user_transactions(
                user_id,
                PageRequestParams {
                    offset: page_count * transactions_per_page,
                    limit: transactions_per_page,
                },
            )
        },
    );

    let error = RwSignal::new(String::new());

    let money_signal = RwSignal::new(Money::default());

    let go_back_padding = "p-5";

    let transaction_signal = RwSignal::new(Vec::<Transaction>::new());

    view! {
        <div>
            <div class=go_back_padding>
                <button class=go_back_padding>
                    <a
                        href=format!("/user/{}", user_id)
                        class=format!("text-white bg-gray-400 rounded {}", go_back_padding)
                    >
                        "Go back"
                    </a>
                </button>
            </div>
            <ShowNavigationButtons
                page_count=page_count
                transaction_signal=transaction_signal
                transactions_per_page=transactions_per_page
            />
            <Transition fallback=move || {
                view! { <h1 class="text-white text-center">"Loading transactions!"</h1> }
            }>
                {move || match trans_resource.get() {
                    None => ().into_any(),
                    Some(value) => {
                        match value {
                            Err(e) => {
                                view! {
                                    <p class="text-red-400">
                                        "Failed to fetch transactions: "{e.to_string()}
                                    </p>
                                }
                                    .into_any()
                            }
                            Ok(value) => {
                                transaction_signal
                                    .update(|transactions| *transactions = value.items);

                                view! {
                                    {transaction_signal
                                        .get()
                                        .iter()
                                        .map(|transaction| {
                                            format_transaction(
                                                transaction,
                                                user_id,
                                                error,
                                                money_signal,
                                            )
                                        })
                                        .collect_view()}
                                }
                                    .into_any()
                            }
                        }
                    }
                }}
            </Transition>
        </div>
        <ShowNavigationButtons
            page_count=page_count
            transaction_signal=transaction_signal
            transactions_per_page=transactions_per_page
        />
    }
    .into_any()
}
