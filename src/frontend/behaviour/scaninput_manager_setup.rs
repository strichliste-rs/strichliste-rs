use leptos::prelude::{Effect, Get, UpdateUntracked};
use leptos_router::hooks::use_url;
use reactive_stores::Store;

use crate::frontend::model::scaninput_manager::ScanInputManager;

impl ScanInputManager {
    pub fn setup(manager: Store<ScanInputManager>) {
        let url = use_url();
        Effect::new(move || {
            let url = url.get();

            manager.update_untracked(move |manager| {
                // kind of bad, but `entry.handle` needs to be an owned value
                if let Some(entries) = manager.entries.take() {
                    let mut new_entries = Vec::new();

                    for entry in entries.into_iter() {
                        // console_log(&format!("url: {} | route: {}", url.path(), entry.route));

                        if url.path() != entry.route {
                            entry.handle.remove();
                            // console_log("removed listener");
                        } else {
                            new_entries.push(entry);
                        }
                    }

                    manager.entries = Some(new_entries);
                }
            });
        });
    }
}
