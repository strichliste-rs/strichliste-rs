use std::rc::Rc;

use thaw::ToasterInjection;

use crate::{
    frontend::{model::money_args::MoneyArgs, shared::create_transaction},
    model::{Money, TransactionType},
};
// fn change_money_logic_raw(money: Money, user_id: UserId, money_signal: RwSignal<Money>, error_signal: RwSignal<String>, transaction_signal: RwSignal<Vec<Transaction>>){
pub fn change_money(money: Money, args: Rc<MoneyArgs>, toaster: ToasterInjection) {
    // this only runs in the main user view

    let (t_type, money) = if money.value > 0 {
        (TransactionType::Deposit, money)
    } else {
        (TransactionType::Withdraw, -money)
    };

    create_transaction(args.clone(), money, t_type, toaster);
}
