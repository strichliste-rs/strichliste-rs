use leptos::prelude::RwSignal;

use crate::model::{Money, Transaction, UserId};

#[derive(Debug, Clone)]
pub struct MoneyArgs {
    pub user_id: UserId,
    pub money: RwSignal<Money>,
    pub transactions: RwSignal<Vec<Transaction>>,
}
