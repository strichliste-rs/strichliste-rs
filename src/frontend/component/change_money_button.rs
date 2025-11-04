use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, ButtonRef, ComponentRef};

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

    let self_ref = ComponentRef::<ButtonRef>::new();

    view! {
        <Button
            appearance=ButtonAppearance::Primary
            class=class
            comp_ref=self_ref
            on_click=move |_| {
                change_money(money.into(), args);
                self_ref.get().unwrap().blur()
            }
        >
            {Money::format_eur_diff_value(money)}
        </Button>
    }
}
