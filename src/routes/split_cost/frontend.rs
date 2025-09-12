use leptos::prelude::*;

use crate::routes::split_cost::components::{
    multi_user_selection::SelectMultiUser, single_user_selection::SelectSingleUser,
};

#[component]
pub fn Show() -> impl IntoView {
    let primary_user = RwSignal::new(String::new());
    let secondary_users = RwSignal::new(Vec::<String>::new());
    let money_input = RwSignal::new(String::new());
    view! {
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
                    <div>
                        <a class="text-indigo-700 w-fit h-fit">"How much?"</a>
                    </div>
                    <div>
                        <input
                            class="ml-5 text-black rounded-[5px] text-center text-indigo-700"
                            type="text"
                            value="0,00"
                            bind:value=money_input
                        />
                    </div>
                </div>
                <div class="flex items-center justify-center bg-indigo-100 rounded p-2">
                    <button class="w-full bg-indigo-700 hover:bg-pink-700 text-white p-3 rounded">"Submit"</button>
                </div>
            </div>
        </div>
    }
}
