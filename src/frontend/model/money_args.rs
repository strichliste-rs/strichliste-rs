use leptos::prelude::{NodeRef, RwSignal};

use crate::model::{Money, Transaction, UserId};

#[derive(Debug, Clone)]
pub struct MoneyArgs {
    pub user_id: UserId,
    pub money: RwSignal<Money>,
    pub error: RwSignal<String>,
    pub transactions: RwSignal<Vec<Transaction>>,
    pub audio_ref: NodeRef<leptos::html::Audio>,
}
