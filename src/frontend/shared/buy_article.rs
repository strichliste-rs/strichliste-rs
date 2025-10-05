use leptos::{prelude::RwSignal, view};
use thaw::{Toast, ToastBody, ToastTitle, ToasterInjection};

use crate::{
    frontend::{model::money_args::MoneyArgs, shared::create_transaction},
    model::Money,
};

pub fn buy_article(
    article_id: i64,
    money: Money,
    args: RwSignal<MoneyArgs>,
    toaster: ToasterInjection,
) {
    create_transaction(
        args,
        money,
        crate::model::TransactionType::Bought(article_id),
        Some(move |transaction: crate::model::Transaction| {
            toaster.dispatch_toast(
                move || {
                    view! {
                        <Toast>
                            <ToastTitle>"Item Bought"</ToastTitle>
                            <ToastBody>
                                "You bought "{transaction.description}" for "
                                {transaction.money.format_eur()}
                            </ToastBody>
                        </Toast>
                    }
                },
                Default::default(),
            );
        }),
    );
}
