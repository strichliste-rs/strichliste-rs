use leptos::prelude::*;
use leptos_use::{use_idle, UseIdleReturn};

/// This creates a component, which navigates to `path` after `seconds` of inactivity.
#[component]
pub fn ReturnTo(
    /// The seconds until it returns
    after: u64,

    /// The route to return to
    route: impl ToString,
) -> impl IntoView {
    let UseIdleReturn { idle, .. } = use_idle(after * 1000);
    let route = route.to_string();

    Effect::new(move || {
        if !idle.get() {
            return;
        }

        let navigate = leptos_router::hooks::use_navigate();

        navigate(&route, Default::default());
    });
}
