use itertools::Itertools;
use leptos::prelude::*;
use thaw::Spinner;

use crate::{
    backend::core::behaviour::user_get_all::get_all_users,
    frontend::{
        component::user::UserPreview, route::home::PREFIX_FILTER_NON_ALPHABETIC_VALUE,
        shared::throw_error_none_view,
    },
};

#[component]
pub fn ShowUsers(filter_prefix: Signal<Option<char>>) -> impl IntoView {
    let user_data = Resource::new(move || {}, |_| get_all_users());

    view! {
        <Suspense fallback=move || {
            view! { <Spinner label="Loading users" /> }
        }>
            {move || {
                let users = match user_data.get() {
                    Some(users) => users,
                    None => {
                        return ().into_any();
                    }
                };
                let users = match users {
                    Ok(users) => users,
                    Err(err) => {
                        return throw_error_none_view(format!("Failed to fetch users: {err}"));
                    }
                };
                let filterd_users = match filter_prefix.get() {
                    Some(filter) => {
                        users
                            .into_iter()
                            .filter_map(|user| {
                                let first_letter = user
                                    .nickname
                                    .chars()
                                    .nth(0)
                                    .expect("nickname isn't allowed empty")
                                    .to_ascii_lowercase();
                                if filter == PREFIX_FILTER_NON_ALPHABETIC_VALUE {
                                    if first_letter.is_alphabetic() {
                                        return None;
                                    }
                                } else if !(first_letter == filter) {
                                    return None;
                                }
                                Some(user)
                            })
                            .collect_vec()
                    }
                    None => users,
                };

                view! {
                    <div class="grid">
                        // manual fix, idk why tailwind does not take grid-cols-[repeat(auto-fill, minmax(8rem, 1fr))]
                        <div
                            class="grid gap-5"
                            style="grid-template-columns: repeat(auto-fill, minmax(8rem, 1fr));"
                        >
                            {filterd_users
                                .into_iter()
                                .map(|user| {
                                    let id = user.id;

                                    view! {
                                        <a href=format!("/user/{}", id)>
                                            <UserPreview user />
                                        </a>
                                    }
                                })
                                .collect_view()
                                .into_any()}
                        </div>
                    </div>
                }
                    .into_any()
            }}
        </Suspense>
    }
}
