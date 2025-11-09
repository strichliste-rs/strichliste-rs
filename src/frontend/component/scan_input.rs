use chrono::Utc;
use leptos::{ev::keypress, prelude::*};
use leptos_use::{use_document, use_event_listener};

const SECONDS_UNTIL_INPUT_CLEARED: i64 = 30;

#[component]
pub fn scan_input<F>(
    ignore_input_signals: Vec<RwSignal<bool>>,
    callback: F,
    #[prop(optional)] should_cleanup: Option<ReadSignal<bool>>,
) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    let input_signal = RwSignal::new(String::new());
    let last_input = RwSignal::new(Utc::now());

    let should_clear_input =
        move || (Utc::now() - last_input.get()).num_seconds() > SECONDS_UNTIL_INPUT_CLEARED;

    let clear_input = move || {
        input_signal.write_only().set(String::new());
    };

    let should_ignore_input = move || ignore_input_signals.iter().any(|elem| elem.get());

    let cleanup = use_event_listener(use_document(), keypress, move |ev| {
        match ev.key().as_str() {
            "Enter" => {
                if should_ignore_input() {
                    return;
                }

                if should_clear_input() {
                    clear_input()
                }

                let scan_input = input_signal.read_untracked().clone();
                clear_input();

                if scan_input.is_empty() {
                    return;
                }

                callback(scan_input);
            }

            _ => {
                if should_ignore_input() {
                    return;
                }

                // Clear input if nothing was typed for 30 seconds
                if should_clear_input() {
                    clear_input()
                }

                input_signal.update_untracked(|string| string.push_str(&ev.key()));

                last_input.write_only().set(Utc::now());
            }
        }
    });

    if let Some(should_cleanup) = should_cleanup {
        Effect::new(move || {
            if should_cleanup.get() {
                cleanup()
            }
        });
    }
}
