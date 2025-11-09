use chrono::Utc;
use leptos::{
    ev,
    prelude::{
        window_event_listener, Get, ReadSignal, ReadUntracked, RwSignal, UpdateUntracked, Write,
    },
};

use crate::frontend::model::scaninput_manager::{
    Entry, ScanInputManager, SECONDS_UNTIL_INPUT_CLEARED,
};

impl ScanInputManager {
    pub fn register<F>(
        &mut self,
        route: impl ToString,
        ignore_input_signals: Vec<ReadSignal<bool>>,
        callback: F,
    ) where
        F: Fn(String) + 'static,
    {
        let route = route.to_string();

        let input_signal = RwSignal::new(String::new());
        let last_input = RwSignal::new(Utc::now());

        let should_clear_input = move || {
            (Utc::now() - last_input.get()).num_seconds() > SECONDS_UNTIL_INPUT_CLEARED as i64
        };

        let clear_input = move || {
            *input_signal.write() = String::new();
        };

        let should_ignore_input = move || {
            ignore_input_signals
                .iter()
                .any(|elem| elem.try_get().unwrap_or(true)) // if signal has been disposed already, just ignore everything
        };

        let cleanup = window_event_listener(ev::keydown, move |ev| {
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

                    *last_input.write() = Utc::now();
                }
            }
        });

        // console_log(&format!("registered listener: {route}"));
        if let Some(mut entries) = self.entries.take() {
            let route_c1 = route.clone();
            entries.retain(move |elem| elem.route != route_c1);
            entries.push(Entry {
                route,
                handle: cleanup,
            });

            self.entries = Some(entries);
        }
    }
}
