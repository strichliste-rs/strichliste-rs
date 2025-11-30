use leptos::prelude::*;

#[component]
pub fn ArrowLeftIcon() -> impl IntoView {
    view! {
        <svg
            width="40"
            height="40"
            viewBox="0 0 40 40"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M20.0002 31.6666L8.3335 20L20.0002 8.33331"
                stroke="#F0F0F0"
                stroke-width="2.5"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
            <path
                d="M31.6668 20H8.3335"
                stroke="#F0F0F0"
                stroke-width="2.5"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
        </svg>
    }
}
