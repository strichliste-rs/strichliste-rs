use std::rc::Rc;

use leptos::prelude::*;

use crate::{frontend::model::money_args::MoneyArgs, model::Money, routes::user::change_money};

pub fn change_money_button(money: i64, args: Rc<MoneyArgs>) -> impl IntoView {
    view! {
        <a
            on:click=move |_| change_money(money.into(), args.clone())
            href="#"
            class="p-5 text-white rounded-[10px] text-center text-[1.25em]"
            class=("bg-emerald-600", move || money > 0)
            class=("bg-red-400", move || money < 0)
        >
            {Money::format_eur_diff_value(money)}
        </a>
    }
}
