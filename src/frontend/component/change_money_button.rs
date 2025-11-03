use std::time::Duration;

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

    let disabled = RwSignal::new(false);

    view! {
        <Button
            appearance=ButtonAppearance::Primary
            class=class
            disabled
            on_click=move |_| {
                disabled.set(true);
                change_money(money.into(), args);
                set_timeout(move || disabled.set(false), Duration::from_millis(100));
            }
        >
            {Money::format_eur_diff_value(money)}
        </Button>
    }
}
