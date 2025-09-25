use leptos::{html, prelude::*};

use crate::model::Transaction;

#[component]
pub fn ShowNavigationButtons(
    page_count: RwSignal<usize>,
    transaction_signal: RwSignal<Vec<Transaction>>,
    transactions_per_page: usize,
) -> impl IntoView {
    view! {
        <div class="flex justify-between p-2">
            {
                let button_ref_prev = NodeRef::<html::Button>::new();
                view! {
                    {move || match transaction_signal.get().len() {
                        0 => {
                            button_ref_prev
                                .get()
                                .map(|elem| { elem.set_attribute("disabled", "") });
                        }
                        _ => {
                            button_ref_prev.get().map(|elem| { elem.remove_attribute("disabled") });
                        }
                    }}
                    <button
                        class="rounded p-5"
                        class=(["bg-gray-400", "text-white"], move || page_count.get() != 0)
                        class=(["bg-white", "text-black"], move || page_count.get() == 0)
                        node_ref=button_ref_prev
                        on:click=move |_| {
                            page_count
                                .update(|value| {
                                    if *value != 0 {
                                        *value -= 1;
                                    }
                                });
                        }
                    >
                        "Previous page"
                    </button>
                    <button
                        class="rounded p-5"
                        class=(
                            ["bg-gray-400", "text-white"],
                            move || transaction_signal.get().len() == transactions_per_page,
                        )
                        class=(
                            ["bg-white", "text-black"],
                            move || transaction_signal.get().len() != transactions_per_page,
                        )
                        on:click=move |_| {
                            let transaction_count = transaction_signal.get_untracked().len();
                            if transaction_count == 0 {
                                return;
                            }
                            if transaction_count < transactions_per_page {
                                return;
                            }
                            page_count.update(|value| *value += 1);
                        }
                    >
                        "Next page"
                    </button>
                }
            }
        </div>
    }
}
