use std::{iter::Scan, str::FromStr};

use leptos::{
    ev::{self, blur, KeyboardEvent, SubmitEvent},
    html,
    leptos_dom::logging::{console_error, console_log},
    prelude::*,
    reactive::signal,
    tachys::{dom::event_target, html::property::IntoProperty},
    task::spawn_local,
};
use leptos_router::hooks::use_navigate;
use leptos_use::{core::IntoElementMaybeSignal, use_event_listener};
use tracing::{debug, error, instrument::WithSubscriber};

use crate::{
    models::User,
    routes::state::{FrontendStore, FrontendStoreStoreFields},
};

#[server]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    use crate::backend::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;

    let response_opts: ResponseOptions = expect_context();

    let users = User::get_all(&*state.db.lock().await).await;
    if users.is_err() {
        let err = users.err().unwrap();
        error!("Could not fetch users: {}", err);
        response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Err(ServerFnError::new(err));
    };

    let users = users.unwrap();

    Ok(users)
}

#[server]
pub async fn get_user_by_barcode(barcode_string: String) -> Result<Option<User>, ServerFnError> {
    use crate::backend::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;

    let response_opts: ResponseOptions = expect_context();

    debug!("Attempting to fetch a user by barcode '{}'", barcode_string);

    if barcode_string.len() == 0 {
        return Ok(None);
    }

    let user = User::get_by_card_number(&*state.db.lock().await, &barcode_string).await;

    if user.is_err() {
        let err = user.err().unwrap();
        error!("Could not fetch user: {}", err);
        response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Err(ServerFnError::new("Failed to fetch user"));
    }

    let user = user.unwrap();

    Ok(user)
}

#[component]
pub fn View() -> impl IntoView {
    view! {
        <div class="grid grid-cols-10 gap-10 py-10">
            <div class="col-span-1 pl-5">
                <a href="/create_user">
                    <div class="flex justify-center">
                        // joinked from: https://gist.github.com/ibelick/0c92c1aba54c2f7e8b3a7381426ed029
                        <button class="inline-flex h-10 w-10 items-center justify-center rounded-full bg-gray-50 text-black drop-shadow-sm transition-colors duration-150 hover:bg-gray-200">
                            "+"
                        </button>
                    </div>
                </a>
            </div>
            {InvisibleScanInput()}
            <div class="col-span-9 pr-7">
                {ShowUsers()}
            </div>
        </div>
    }
}

#[component]
pub fn InvisibleScanInput() -> impl IntoView {
    let input_signal = RwSignal::new(String::new());

    let handle = window_event_listener(ev::keypress, move |ev| match ev.key().as_str() {
        "Enter" => {
            let scan_input = input_signal.read_untracked().clone();
            input_signal.write_only().set(String::new());

            if scan_input.len() == 0 {
                return;
            }

            spawn_local(async move {
                let user = get_user_by_barcode(scan_input.clone()).await;
                if user.is_err() {
                    console_log(&format!(
                        "Failed to fetch user by barcode: {}",
                        user.err().unwrap()
                    ));
                    return;
                }

                let user = user.unwrap();
                if user.is_none() {
                    console_log(&format!("There is no user with barcode \"{}\"", scan_input));
                    return;
                }

                let user = user.unwrap();
                let navigate = use_navigate();
                navigate(&format!("/user/{}", user.id.unwrap()), Default::default());
            });
        }

        _ => {
            let mut prev = input_signal.read_untracked().clone();
            prev.push_str(&ev.key());
            input_signal.write_only().set(prev);
        }
    });

    on_cleanup(move || {
        handle.remove();
    });

    // scan_input_field.on_load(|elem| {
    //     _ = elem.focus();
    // });

    return view! {
        // <input id="invisible_barcode_input" type="text" autofocus style="position: absolute; left: -9999px;"
        // node_ref=scan_input_field
        // // on:submit=on_submit
        // on:blur=move |_| {
        //     console_log("input is out of focus");
        //     if let Some(input) = scan_input_field.get() {
        //         _ = input.focus();
        //         console_log("input element should be in focus");
        //     }
        // }
        // />
    };
}

#[component]
pub fn ShowUsers() -> impl IntoView {
    // use reactive_stores::Store;
    // let store = expect_context::<Store<FrontendStore>>();

    // let fetch_users = RwSignal::new(0 as i64);

    let user_data = Resource::new(move || {}, |_| get_users());

    view! {
        <Suspense
            fallback=move ||view! { <h1>"Loading users..."</h1>}
        >
            { move || {

                let users = user_data.get();

                if users.is_none() {
                    return view!{
                        <p class="text-red-900">"Failed to fetch users"</p>
                    }.into_any();
                }

                let users = users.unwrap();

                if users.is_err() {
                    let error = users.err().unwrap().to_string();
                    return view!{
                        <p class="text-red-900">"Failed to fetch users: "{error}</p>
                    }.into_any();
                }

                let users = users.unwrap();

                // store.cached_users().writer().unwrap().clear();

                // store.cached_users().writer().unwrap().append(&mut users.clone());

                view!{
                    <div class="grid">
                    // manual fix, idk why tailwind does not take grid-cols-[repeat(auto-fill, minmax(8rem, 1fr))]
                    <div class="grid gap-5" style="grid-template-columns: repeat(auto-fill, minmax(8rem, 1fr));">
                        {
                            users.into_iter().map(|user| {

                                let id = user.id.clone();

                                view!{
                                    <a href=format!("/user/{}", id.unwrap())>
                                        <UserPreview user/>
                                    </a>
                                }
                            }).collect_view().into_any()
                        }
                    </div>
                    </div>
                }.into_any()

            }
        }
        </Suspense>
    }
}

#[component]
pub fn UserPreview(user: User) -> impl IntoView {
    view! {
        <div class="flex flex-col bg-[#2e3d4d] gap-2 rounded-[10px] py-2">
            <p class="text-center text-white">{user.nickname.clone()}</p>
            <p class="text-center"
                class=("text-red-500", move || {user.money < 0})
                class=("text-green-500", move ||{user.money >= 0})
            >{user.get_money()}"€"</p>
        </div>
    }
}
