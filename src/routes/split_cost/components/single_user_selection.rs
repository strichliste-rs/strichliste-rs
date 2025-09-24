use leptos::{html, prelude::*};

use crate::{
    backend::core::behaviour::user_get_all::get_all_users,
    routes::components::error_message::ErrorMessage,
};

const MAX_USER_DISPLAY_LIMIT: usize = 5;

#[component]
pub fn SelectSingleUser(
    title: String,
    input: RwSignal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional)] extra_class: Option<String>,
    #[prop(optional)] filter_names: Option<RwSignal<Vec<String>>>,
) -> impl IntoView {
    let hidden_div_node_ref = NodeRef::<html::Div>::new();

    let all_users_resource = OnceResource::new(get_all_users());

    view! {
        <Suspense fallback=move || {
            view! { <p class="text-white text-center">"Loading users"</p> }
        }>
            {move || {
                let all_users = match all_users_resource.get() {
                    Some(Ok(value)) => value,
                    _ => {
                        return view! {
                            <ErrorMessage error=String::from("Failed to fetch users!") />
                        }
                            .into_any();
                    }
                };

                view! {
                    <div class=format!("flex {}", extra_class.as_ref().map_or("", |v| v))>
                        <div class=format!(
                            "bg-indigo-100 rounded p-5 {}",
                            extra_class.as_ref().map_or("", |v| v),
                        )>
                            <div>
                                <label class="block mb-2 text-indigo-500">{title.clone()}</label>
                                <input
                                    bind:value=input
                                    autocomplete="off"
                                    class="w-full p-2 mb-6 text-indigo-700 border-b-2 border-indigo-500 outline-none focus:bg-gray-300"
                                    type="text"
                                    name="to_user"
                                />
                            </div>
                            <div
                                class="flex flex-col items-center gap-2"
                                class=("hidden", move || input.get().is_empty())
                                node_ref=hidden_div_node_ref
                            >
                                {move || {
                                    let search = input.get();
                                    all_users
                                        .iter()
                                        .filter(|elem| {
                                            elem.nickname
                                                .to_lowercase()
                                                .contains(&search.to_lowercase())
                                        })
                                        .filter(|elem| {
                                            match filter_names {
                                                None => true,
                                                Some(signal) => {
                                                    let value = signal.get();
                                                    for name in value.iter() {
                                                        if *name == *elem.nickname {
                                                            return false;
                                                        }
                                                    }
                                                    true
                                                }
                                            }
                                        })
                                        .take(MAX_USER_DISPLAY_LIMIT)
                                        .map(|elem| {
                                            let nickname = elem.nickname.clone();
                                            let n_clone = nickname.clone();

                                            view! {
                                                <button
                                                    class="bg-gray-400 text-white p-2 rounded"
                                                    on:click=move |_| {
                                                        input.set(n_clone.clone());
                                                        hidden_div_node_ref
                                                            .get()
                                                            .map(|elem| {
                                                                elem.class("hidden flex flex-col items-center")
                                                            });
                                                    }
                                                >
                                                    {nickname}
                                                </button>
                                            }
                                        })
                                        .collect_view()
                                }}
                            </div>
                            {children.as_ref().map(|children| children())}
                        </div>
                    </div>
                }
                    .into_any()
            }}
        </Suspense>
    }
}
