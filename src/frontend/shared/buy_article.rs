use leptos::{leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use thaw::{Toast, ToastBody, ToastTitle, ToasterInjection};

use crate::{
    backend::core::{behaviour::buy_article::buy_article_by_id, Article},
    frontend::shared::{play_sound, throw_error},
    model::{AudioPlayback, Money, Transaction, UserId},
};

pub fn buy_article(
    user_id: UserId,
    article: Article,
    money: RwSignal<Money>,
    transactions: RwSignal<Vec<Transaction>>,
    toaster: ToasterInjection,
) {
    console_log(&format!("Need to buy article with id: {}", article.id));
    spawn_local(async move {
        match buy_article_by_id(user_id, article.id).await {
            Ok(transaction) => {
                money.update(|money| money.value -= transaction.money.value);
                transactions.update(|trns| trns.insert(0, transaction));
                play_sound(AudioPlayback::Bought(article.id));
                toaster.dispatch_toast(
                    move || {
                        view! {
                            <Toast>
                                <ToastTitle>"Item Bought"</ToastTitle>
                                <ToastBody>
                                    "You bought "{article.name}" for " {article.cost.format_eur()}
                                </ToastBody>
                            </Toast>
                        }
                    },
                    Default::default(),
                );
            }

            Err(e) => {
                throw_error(format!("Failed to buy article: {e}"));
            }
        }
    });
}
