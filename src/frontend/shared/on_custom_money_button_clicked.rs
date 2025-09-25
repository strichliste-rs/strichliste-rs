use std::rc::Rc;

use leptos::prelude::*;

use crate::{
    frontend::{model::money_args::MoneyArgs, shared::change_money},
    model::Money,
};

pub fn on_custom_money_button_click(add: bool, value: RwSignal<String>, args: &MoneyArgs) {
    let string = value.get_untracked();

    let error_signal = args.error;
    error_signal.set(String::new());

    if string.is_empty() {
        return;
    }

    let mut money: Money = match string.try_into() {
        Ok(value) => value,
        Err(e) => {
            error_signal.set(format!("Failed to parse money: {e}"));
            return;
        }
    };

    if money.value == 0 {
        return;
    }

    if !add {
        money.value = -money.value;
    }

    change_money(money, Rc::new(args.clone()));

    value.set(String::new());
}
