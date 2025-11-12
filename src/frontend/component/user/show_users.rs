use itertools::Itertools;
use leptos::prelude::*;
use thaw::Spinner;

use crate::frontend::{
    component::user::UserPreview,
    model::{
        caching_layer::CachingLayer,
        frontend_store::{FrontendStoreStoreFields, FrontendStoreType},
    },
    route::home::PREFIX_FILTER_NON_ALPHABETIC_VALUE,
    shared::throw_error,
};

#[component]
pub fn ShowUsers(prefix_filter: Signal<Option<char>>) -> impl IntoView {
    let store = expect_context::<FrontendStoreType>();

    let cache = store.cachinglayer().get_untracked();

    let users = CachingLayer::get_all_users(cache);

    Effect::new(move || {
        if let Some(error) = users.read().error.get() {
            throw_error(error);
        }
    });

    view! {
        {move || {
            if users.read().is_fetching.get() && users.read().value.get().is_empty() {
                return view! { <Spinner label="Loading users!" /> }.into_any();
            }
            let filterd_users = match prefix_filter.get() {
                Some(filter) => {
                    users
                        .read()
                        .value
                        .get()
                        .into_iter()
                        .filter_map(|user| {
                            let first_letter = user
                                .nickname
                                .chars()
                                .next()
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
                None => users.read().value.get(),
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
                                Some(

                                    view! {
                                        <a href=format!("/user/{}", id.0)>
                                            <UserPreview user />
                                        </a>
                                    },
                                )
                            })
                            .collect_view()
                            .into_any()}
                    </div>
                </div>
            }
                .into_any()
        }}
    }
}
