use leptos::prelude::*;

#[component]
pub fn RotateLeftIcon() -> impl IntoView {
    view! {
        <svg
            width="40"
            height="40"
            viewBox="0 0 40 40"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M5 20C5 22.9667 5.87973 25.8668 7.52796 28.3336C9.17618 30.8003 11.5189 32.7229 14.2597 33.8582C17.0006 34.9935 20.0166 35.2906 22.9264 34.7118C25.8361 34.133 28.5088 32.7044 30.6066 30.6066C32.7044 28.5088 34.133 25.8361 34.7118 22.9264C35.2906 20.0166 34.9935 17.0006 33.8582 14.2597C32.7229 11.5189 30.8003 9.17618 28.3336 7.52796C25.8668 5.87973 22.9667 5 20 5C15.8066 5.01578 11.7816 6.65204 8.76667 9.56667L5 13.3333"
                stroke="#F0F0F0"
                stroke-width="2.5"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
            <path
                d="M5 5V13.3333H13.3333"
                stroke="#F0F0F0"
                stroke-width="2.5"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
        </svg>
    }
}
