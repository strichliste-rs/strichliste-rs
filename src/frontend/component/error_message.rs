use leptos::prelude::*;

#[component]
pub fn ErrorMessage(error: String) -> impl IntoView {
    view! {
        <div>
            <p class="bg-red-400 text-white text-center">{error}</p>
        </div>
    }
}
