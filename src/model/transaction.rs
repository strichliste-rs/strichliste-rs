use chrono::{DateTime, Utc};
use leptos::prelude::RwSignal;
use serde::{Deserialize, Serialize};

use crate::model::{DatabaseId, GroupId, Money};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub id: DatabaseId,
    /// used to look up name (for split transaction)
    pub group_id: GroupId,
    pub is_undone: bool,
    pub t_type: TransactionType,
    pub money: Money,
    pub description: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub is_undone_signal: RwSignal<bool>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionType {
    Deposit,
    Withdraw,
    Bought(i64),
    Received(GroupId),
    Sent(GroupId),
    SentAndReceived(GroupId), // sending group is stored as group_id in Transaction
}

#[cfg(feature = "ssr")]
pub struct TransactionDelta {
    pub(crate) amount_pre: i64,
    pub(crate) delta: i64,
}

#[cfg(feature = "ssr")]
impl TransactionDelta {
    pub fn post_amount(&self) -> i64 {
        self.amount_pre + self.delta
    }
}
