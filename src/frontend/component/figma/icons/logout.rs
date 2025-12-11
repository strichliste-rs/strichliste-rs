use leptos::prelude::*;

#[component]
pub fn logout_icon() -> impl IntoView {
    view! {
        <svg width="40" height="40" viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M26.6667 28.3334L35.0001 20L26.6667 11.6667" stroke="#F0F0F0" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M35 20H15" stroke="#F0F0F0" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M15 35H8.33333C7.44928 35 6.60143 34.6488 5.97631 34.0237C5.35119 33.3986 5 32.5507 5 31.6667V8.33333C5 7.44928 5.35119 6.60143 5.97631 5.97631C6.60143 5.35119 7.44928 5 8.33333 5H15" stroke="#F0F0F0" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>

    }
}
