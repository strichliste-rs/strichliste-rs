use std::rc::Rc;

use leptos::{leptos_dom::logging::console_log, prelude::*, task::spawn_local};

use crate::{
    backend::core::behaviour::buy_article::buy_article_by_id,
    frontend::{model::money_args::MoneyArgs, shared::play_sound},
    model::{AudioPlayback, Money, Transaction, UserId},
};

pub fn buy_article(
    user_id: UserId,
    article_id: i64,
    money: RwSignal<Money>,
    error: RwSignal<String>,
    transactions: RwSignal<Vec<Transaction>>,
    audio_ref: NodeRef<leptos::html::Audio>,
) {
    console_log(&format!("Need to buy article with id: {article_id}"));
    let args = MoneyArgs {
        user_id,
        money,
        error,
        transactions,
        audio_ref,
    };
    spawn_local(async move {
        match buy_article_by_id(user_id, article_id).await {
            Ok(transaction) => {
                money.update(|money| money.value -= transaction.money.value);
                transactions.update(|trns| trns.insert(0, transaction));
                error.set(String::new());
                play_sound(Rc::new(args), AudioPlayback::Bought(article_id));
            }

            Err(e) => {
                error.set(format!("Failed to buy article: {e}"));
            }
        }
    });
}
