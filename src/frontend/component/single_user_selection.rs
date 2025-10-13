use leptos::prelude::*;
use thaw::{AutoComplete, AutoCompleteOption, Field};

use crate::{
    backend::core::behaviour::user_get_all::get_all_users,
    frontend::component::error_message::ErrorMessage,
};

const MAX_USER_DISPLAY_LIMIT: usize = 5;

#[component]
pub fn SelectSingleUser(
    title: String,
    input: RwSignal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional)] filter_names: Option<RwSignal<Vec<String>>>,
) -> impl IntoView {
    let all_users_resource = OnceResource::new(get_all_users());

    let title = RwSignal::new(title);

    view! {
        <Suspense fallback=move || {
            view! { <p class="text-white text-center">"Loading users"</p> }
        }>
            {move || {
                let all_users = match all_users_resource.get() {
                    Some(Ok(value)) => RwSignal::new(value),
                    _ => {
                        return view! {
                            <ErrorMessage error=String::from("Failed to fetch users!") />
                        }
                            .into_any();
                    }
                };
                let options = Memo::<
                    Vec<String>,
                >::new(move |_| {
                    all_users
                        .get()
                        .iter()
                        .filter(|elem| {
                            elem.nickname.to_lowercase().contains(&input.get().to_lowercase())
                        })
                        .filter(|elem| {
                            match filter_names {
                                Some(names) => {
                                    for name in names.get().iter() {
                                        if name == &elem.nickname {
                                            return false;
                                        }
                                    }
                                    true
                                }
                                None => true,
                            }
                        })
                        .map(|elem| elem.nickname.clone())
                        .take(MAX_USER_DISPLAY_LIMIT)
                        .collect()
                });

                view! {
                    <Field label=title>
                        <AutoComplete value=input>
                            <For each=move || options.get() key=|option| option.clone() let:option>

                                <AutoCompleteOption value=option
                                    .clone()>{option}</AutoCompleteOption>
                            </For>

                        </AutoComplete>
                    </Field>
                    {children.as_ref().map(|children| children())}
                }
                    .into_any()
            }}
        </Suspense>
    }
}
