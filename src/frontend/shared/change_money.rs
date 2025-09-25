use std::rc::Rc;

use leptos::{prelude::*, task::spawn_local};

use crate::{
    backend::core::behaviour::transaction_create::create_transaction,
    frontend::{model::money_args::MoneyArgs, shared::play_sound},
    model::{AudioPlayback, CreateTransactionError, Money, TransactionType},
};
// fn change_money_logic_raw(money: Money, user_id: UserId, money_signal: RwSignal<Money>, error_signal: RwSignal<String>, transaction_signal: RwSignal<Vec<Transaction>>){
pub fn change_money(money: Money, args: Rc<MoneyArgs>) {
    // this only runs in the main user view
    spawn_local(async move {
        let mut fixed_money = money;
        let t_type = if money.value > 0 {
            TransactionType::Deposit
        } else {
            fixed_money = -fixed_money;
            TransactionType::Withdraw
        };

        match create_transaction(args.user_id, fixed_money, t_type).await {
            Ok(transaction) => {
                args.money
                    .update(|money_struct| money_struct.value += money.value);
                args.error.set(String::new());
                args.transactions.write().insert(0, transaction.clone());
                play_sound(
                    args.clone(),
                    match transaction.t_type {
                        TransactionType::Bought(id) => AudioPlayback::Bought(id),
                        TransactionType::Deposit => AudioPlayback::Deposit(transaction.money),
                        TransactionType::Withdraw => AudioPlayback::Withdraw(transaction.money),
                        TransactionType::Received(_) => return,
                        TransactionType::SentAndReceived(_) => return,
                        TransactionType::Sent(_) => AudioPlayback::Sent(transaction.money),
                    },
                );
            }
            Err(e) => {
                let msg = match e {
                    CreateTransactionError::TooLittleMoneyError(_) => {
                        "You have too little money!".to_string()
                    }
                    CreateTransactionError::TooMuchMoneyError(_) => {
                        "You have too much money!".to_string()
                    }
                    CreateTransactionError::StringMessage(msg) => msg,
                    CreateTransactionError::ServerFn(server_fn) => server_fn.to_string(),
                };
                args.error.set(msg);
                play_sound(args.clone(), AudioPlayback::Failed);
            }
        };
    })
}
