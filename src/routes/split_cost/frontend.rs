use leptos::{prelude::*, task::spawn_local};

use crate::routes::{
    components::error_message::ErrorMessage,
    split_cost::{
        components::{
            multi_user_selection::SelectMultiUser, single_user_selection::SelectSingleUser,
        },
        split_cost,
    },
};

#[component]
pub fn Show() -> impl IntoView {
    let primary_user = RwSignal::new(String::new());
    let secondary_users = RwSignal::new(Vec::<String>::new());
    let money_input = RwSignal::new(String::new());
    let description_input = RwSignal::new(String::new());

    let error_signal = RwSignal::new(String::new());

    // don't see how I can pass a Vec<String> to a server function with ActionForms
    let on_click = move |_| {
        spawn_local(async move {
            if let Err(e) = split_cost(
                primary_user.get_untracked(),
                Some(secondary_users.get_untracked()),
                money_input.get_untracked(),
                description_input.get_untracked(),
            )
            .await
            {
                error_signal.update(|value| *value = e.to_string());
            }
        })
    };
    view! {
        {move || {
            let msg = error_signal.get();
            match msg.len() {
                0 => ().into_any(),
                _ => view! { <ErrorMessage error=msg /> }.into_any(),
            }
        }}
        <div class="flex flex-col items-center text-[1.25em]">
            <div class="grid grid-cols-2 py-2 w-fit h-fit justify-center gap-2">
                <SelectSingleUser
                    title=String::from("Who are you?")
                    input=primary_user
                    extra_class="w-full h-full".to_string()
                />
                <SelectMultiUser
                    title=String::from("Who do you want to split the cost with?")
                    users_input=secondary_users
                    single_user_extra_class="w-full h-full".to_string()
                />
                <div class="flex justify-center items-center p-2 bg-indigo-100 rounded">
                    <div class="flex flex-col items-center gap-3">
                        <a class="text-indigo-700 w-fit h-fit">"How much?"</a>
                        <a class="text-indigo-700 w-fit h-fit">"Description:"</a>
                    </div>
                    <div class="flex flex-col items-center gap-3">
                        <input
                            class="ml-5 text-black rounded-[5px] text-center text-indigo-700"
                            type="text"
                            value="0,00"
                            bind:value=money_input
                        />
                        <input
                            class="ml-5 text-black rounded-[5px] text-center text-indigo-700"
                            type="text"
                            value=""
                            bind:value=description_input
                        />
                    </div>
                </div>
                <div class="flex items-center justify-center bg-indigo-100 rounded p-2">
                    <button
                        class="w-full bg-indigo-700 hover:bg-pink-700 text-white p-3 rounded"
                        on:click=on_click
                    >
                        "Split cost"
                    </button>
                </div>
            </div>
        </div>
    }
}
