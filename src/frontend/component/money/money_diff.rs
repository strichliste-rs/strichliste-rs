use leptos::prelude::*;

use crate::model::Money;

#[component]
pub fn money_diff(money: Money) -> impl IntoView {
    view! {
        <p
            class=("text-red-400", move || money.value < 0)
            class=("text-green-500", move || money.value >= 0)
        >
            {money.format_eur_diff()}
        </p>
    }
}
