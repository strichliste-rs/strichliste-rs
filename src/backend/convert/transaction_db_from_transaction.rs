#![cfg(feature = "ssr")]

use crate::{
    backend::database::{TransactionDB, DBGROUP_AUFLADUNG_ID, DBGROUP_SNACKBAR_ID},
    model::Transaction,
};
impl From<Transaction> for TransactionDB {
    fn from(value: Transaction) -> Self {
        use crate::model::TransactionType;

        let Transaction {
            id,
            is_undone,
            t_type,
            money,
            description,
            timestamp,
            is_undone_signal: _,
            group_id,
        } = value;

        let (sender, receiver) = match t_type {
            TransactionType::Deposit => (group_id, DBGROUP_AUFLADUNG_ID),
            TransactionType::Withdraw => (DBGROUP_AUFLADUNG_ID, group_id),
            TransactionType::Bought(_) => (group_id, DBGROUP_SNACKBAR_ID),
            TransactionType::Received(from) => (from, group_id),
            TransactionType::Sent(to) => (group_id, to),
            TransactionType::SentAndReceived(to) => (group_id, to),
        };

        Self {
            id,
            sender: sender.0,
            receiver: receiver.0,
            is_undone,
            t_type_data: match t_type {
                TransactionType::Sent(var)
                | TransactionType::SentAndReceived(var)
                | TransactionType::Received(var) => Some(var.0),

                TransactionType::Bought(var) => Some(var),
                _ => None,
            },
            money: money.value as u64,
            description,
            timestamp,
        }
    }
}
