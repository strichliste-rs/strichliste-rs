use leptos::{ev, leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use leptos_router::hooks::use_navigate;

use crate::backend::core::behaviour::user_get_by_card_number::get_user_by_barcode;

#[component]
pub fn InvisibleScanInput() -> impl IntoView {
    let input_signal = RwSignal::new(String::new());

    let handle = window_event_listener(ev::keypress, move |ev| match ev.key().as_str() {
        "Enter" => {
            let scan_input = input_signal.read_untracked().clone();
            input_signal.write_only().set(String::new());

            if scan_input.is_empty() {
                return;
            }

            spawn_local(async move {
                let user = match get_user_by_barcode(scan_input.clone()).await {
                    Ok(user) => user,
                    Err(err) => {
                        console_log(&format!("Failed to fetch user by barcode: {}", err));
                        return;
                    }
                };

                let user = match user {
                    Some(user) => user,
                    None => {
                        console_log(&format!("There is no user with barcode \"{scan_input}\""));
                        return;
                    }
                };

                let navigate = use_navigate();
                navigate(&format!("/user/{}", user.id), Default::default());
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
}
