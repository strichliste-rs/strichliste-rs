use leptos::prelude::*;

use crate::{backend::core::User, model::Money};

#[component]
pub fn UserPreview(user: User) -> impl IntoView {
    view! {
        <div class="flex flex-col bg-[#2e3d4d] gap-2 rounded-[10px] py-2">
            <p class="text-center text-white">{user.nickname.clone()}</p>
            <p
                class="text-center"
                class=("text-red-500", move || { user.money.value < 0 })
                class=("text-green-500", move || { user.money.value >= 0 })
            >
                {Money::format_eur_diff_value(user.money.value)}
            </p>
        </div>
    }
}
