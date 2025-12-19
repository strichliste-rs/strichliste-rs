use leptos::prelude::*;

#[component]
pub fn split_way_icon() -> impl IntoView {
    view! {
        <svg width="40" height="40" viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M26.6667 5H35.0001V13.3333" stroke="#F0F0F0" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M13.3333 5H5V13.3333" stroke="#F0F0F0" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M20 36.6667V22.8333C20.0095 21.9456 19.8416 21.0649 19.5062 20.2429C19.1708 19.421 18.6746 18.6743 18.0467 18.0467L5 5" stroke="#F0F0F0" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M25 15L35 5" stroke="#F0F0F0" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>

    }
}
