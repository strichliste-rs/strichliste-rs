use leptos::prelude::*;

#[component]
pub fn View() -> impl IntoView {
    view! {
        <nav> // navbar
            <div class="flex flex-row gap-4 p-8 bg-[#1d2832] text-gray-200">
                <a href="/">"Strichliste"</a>
                <a href="/inactive_users">"Inactive users"</a>
                <a href="/articles">"Article list"</a>
            </div>
        </nav>
    }
}
