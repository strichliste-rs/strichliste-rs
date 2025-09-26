use leptos::{leptos_dom::logging::console_log, prelude::*, task::spawn_local};

use crate::{
    backend::core::behaviour::buy_article::buy_article_by_id,
    frontend::shared::{play_sound, throw_error},
    model::{AudioPlayback, Money, Transaction, UserId},
};

pub fn buy_article(
    user_id: UserId,
    article_id: i64,
    money: RwSignal<Money>,
    transactions: RwSignal<Vec<Transaction>>,
) {
    console_log(&format!("Need to buy article with id: {article_id}"));
    spawn_local(async move {
        match buy_article_by_id(user_id, article_id).await {
            Ok(transaction) => {
                money.update(|money| money.value -= transaction.money.value);
                transactions.update(|trns| trns.insert(0, transaction));
                play_sound(AudioPlayback::Bought(article_id));
            }

            Err(e) => {
                throw_error(format!("Failed to buy article: {e}"));
            }
        }
    });
}
