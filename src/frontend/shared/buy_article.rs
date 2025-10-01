use std::rc::Rc;

use thaw::ToasterInjection;

use crate::{
    frontend::{model::money_args::MoneyArgs, shared::create_transaction},
    model::Money,
};

pub fn buy_article(article_id: i64, money: Money, args: Rc<MoneyArgs>, toaster: ToasterInjection) {
    create_transaction(
        args,
        money,
        crate::model::TransactionType::Bought(article_id),
        toaster,
    );
}
