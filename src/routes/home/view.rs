use leptos::prelude::*;

#[component]
pub fn View() -> impl IntoView {
    view! {
        <div class="grid grid-cols-10 gap-10">
            <div class="bg-blue-900 col-span-1">
                // button
                <p>"Button"</p>
            </div>
            <div class="flext justify-start col-span-9">
                // content
                <p>"Content"</p>
            </div>
        </div>
    }
}

#[component]
pub fn ShowUsers() -> impl IntoView {}
