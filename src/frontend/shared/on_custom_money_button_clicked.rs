use leptos::prelude::*;

use crate::{
    frontend::{
        model::money_args::MoneyArgs,
        shared::{change_money, throw_error},
    },
    model::Money,
};

pub fn on_custom_money_button_click(add: bool, value: RwSignal<String>, args: RwSignal<MoneyArgs>) {
    let string = value.get_untracked();

    if string.is_empty() {
        return;
    }

    let mut money: Money = match string.try_into() {
        Ok(value) => value,
        Err(e) => {
            throw_error(format!("Failed to parse money: {e}"));
            return;
        }
    };

    if money.value == 0 {
        return;
    }

    if !add {
        money.value = -money.value;
    }

    change_money(money, args);

    value.set(String::new());
}
