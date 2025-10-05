use leptos::prelude::*;
use thaw::{Button, ButtonAppearance};

use crate::{
    frontend::{model::money_args::MoneyArgs, shared::change_money},
    model::Money,
};

#[component]
pub fn ChangeMoneyButton(money: i64, args: RwSignal<MoneyArgs>) -> impl IntoView {
    let class = if money > 0 {
        "bg-emerald-600"
    } else {
        "bg-red-400"
    }
    .to_owned()
        + " p-5";
    view! {
        <Button
            appearance=ButtonAppearance::Primary
            class=class
            on_click=move |_| change_money(money.into(), args)
        >
            {Money::format_eur_diff_value(money)}
        </Button>
    }
}
