use std::rc::Rc;

use leptos::{
    leptos_dom::logging::console_log,
    prelude::{Update, Write},
    reactive::spawn_local,
};

use crate::{
    backend::core::behaviour::transaction_create::create_transaction as server_create_transaction,
    frontend::{
        model::money_args::MoneyArgs,
        shared::{play_sound, throw_error},
    },
    model::{AudioPlayback, CreateTransactionError, Money, Transaction, TransactionType},
};

pub fn create_transaction(
    user_args: Rc<MoneyArgs>,
    money: Money,
    transaction_type: TransactionType,
    on_success: Option<impl Fn(Transaction) + Send + Sync + 'static>,
) {
    if (money.value) < 0 {
        console_log("Money may not be negative!");
        return;
    }

    spawn_local(async move {
        match server_create_transaction(user_args.user_id, money, transaction_type).await {
            Ok((transaction, user_diff)) => {
                user_args
                    .money
                    .update(|money_prev| *money_prev += user_diff);

                user_args
                    .transactions
                    .write()
                    .insert(0, transaction.clone());

                play_sound(match transaction.t_type {
                    TransactionType::Bought(id) => AudioPlayback::Bought(id),
                    TransactionType::Deposit => AudioPlayback::Deposit(transaction.money),
                    TransactionType::Withdraw => AudioPlayback::Withdraw(transaction.money),
                    TransactionType::Received(_) => return,
                    TransactionType::SentAndReceived(_) => return,
                    TransactionType::Sent(_) => AudioPlayback::Sent(transaction.money),
                });

                if let Some(on_success_fn) = on_success {
                    on_success_fn(transaction)
                }
            }

            Err(e) => {
                let msg = match e {
                    CreateTransactionError::TooLittleMoneyError(_) => {
                        "You have too little money!".to_string()
                    }
                    CreateTransactionError::TooMuchMoneyError(_) => {
                        "You have too much money!".to_string()
                    }

                    _ => e.to_string(),
                };
                throw_error(msg);
                play_sound(AudioPlayback::Failed);
            }
        }
    })
}
