use leptos::prelude::*;
use thaw::Spinner;

use crate::frontend::{
    component::user::UserPreview,
    model::{
        caching_layer::CachingLayer,
        frontend_store::{FrontendStoreStoreFields, FrontendStoreType},
    },
    shared::throw_error,
};

#[component]
pub fn ShowUsers() -> impl IntoView {
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

            view! {
                <div class="grid">
                    // manual fix, idk why tailwind does not take grid-cols-[repeat(auto-fill, minmax(8rem, 1fr))]
                    <div
                        class="grid gap-5"
                        style="grid-template-columns: repeat(auto-fill, minmax(8rem, 1fr));"
                    >
                        {users
                            .read()
                            .value
                            .get()
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
    }
}
