use leptos::prelude::*;

use crate::{
    backend::core::behaviour::user_get_all::get_all_users,
    frontend::{component::user::UserPreview, shared::throw_error_none_view},
};

#[component]
pub fn ShowUsers() -> impl IntoView {
    // use reactive_stores::Store;
    // let store = expect_context::<Store<FrontendStore>>();

    // let fetch_users = RwSignal::new(0 as i64);

    let user_data = Resource::new(move || {}, |_| get_all_users());

    view! {
        <Suspense fallback=move || {
            view! { <h1>"Loading users..."</h1> }
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

                // store.cached_users().writer().unwrap().clear();

                // store.cached_users().writer().unwrap().append(&mut users.clone());

                view! {
                    <div class="grid">
                        // manual fix, idk why tailwind does not take grid-cols-[repeat(auto-fill, minmax(8rem, 1fr))]
                        <div
                            class="grid gap-5"
                            style="grid-template-columns: repeat(auto-fill, minmax(8rem, 1fr));"
                        >
                            {users
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
