use leptos::prelude::*;

#[component]
pub fn LeftRightArrowIcon(class: &'static str) -> impl IntoView {
    view! {
        <div class=class>
            <svg
                viewBox="0 0 24 24"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                transform="matrix(1, 0, 0, 1, 0, 0)rotate(90)"
            >
                <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
                <g
                    id="SVGRepo_tracerCarrier"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke="#CCCCCC"
                    stroke-width="0.4800000000000001"
                ></g>
                <g id="SVGRepo_iconCarrier">
                    <path
                        d="M7 4V20M7 20L3 16M7 20L11 16M17 4V20M17 4L21 8M17 4L13 8"
                        stroke="#ffffff"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        style="--darkreader-inline-stroke: var(--darkreader-text-000000, #e8e6e3);"
                        data-darkreader-inline-stroke=""
                    ></path>
                </g>
            </svg>
        </div>
    }
}
