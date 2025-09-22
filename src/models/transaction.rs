#[cfg(feature = "ssr")]
use crate::{
    backend::database::TransactionDB,
    model::{GroupId, Transaction},
};

#[cfg(feature = "ssr")]
use {
    crate::backend::database::DBError,
    crate::backend::database::{DBGROUP_AUFLADUNG_ID, DBGROUP_SNACKBAR_ID},
    itertools::Itertools,
    leptos::prelude::RwSignal,
};

#[cfg(feature = "ssr")]
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

#[cfg(feature = "ssr")]
/// Use the GroupId (self.1) if the user is the only person relevant in the
/// transaction
impl<'a, T> TryInto<Transaction> for (TransactionDB, &'a Vec<T>)
where
    for<'b> &'b T: Into<GroupId>,
{
    type Error = DBError;
    fn try_into(self: (TransactionDB, &'a Vec<T>)) -> Result<Transaction, DBError> {
        use crate::{backend::database::DBGROUP_AUFLADUNG_ID, model::Transaction};

        let (
            TransactionDB {
                id,
                sender,
                receiver,
                is_undone,
                t_type_data,
                money,
                description,
                timestamp,
            },
            group_ids,
        ) = self;
        let (sender, receiver) = (GroupId(sender), GroupId(receiver));

        let group_ids = group_ids.iter().map(Into::<GroupId>::into).collect_vec();

        let is_sender = group_ids.contains(&sender);

        let is_receiver = group_ids.contains(&receiver);

        Ok(Transaction {
            id,
            group_id: match (is_sender, is_receiver) {
                (true, true) => sender,
                (true, false) => sender,
                (false, true) => receiver,
                (false, false) => {
                    return Err(DBError::new(
                        "invalid state when converting TransactionDB to Transaction either sender or reciever must be group id",
                    ));
                }
            },
            is_undone,
            t_type: {
                use crate::{backend::database::DBGROUP_SNACKBAR_ID, model::TransactionType};
                match (sender, receiver) {
                    (DBGROUP_AUFLADUNG_ID, _) => TransactionType::Deposit,
                    (_, DBGROUP_AUFLADUNG_ID) => TransactionType::Withdraw,
                    (_, DBGROUP_SNACKBAR_ID) => TransactionType::Bought(t_type_data.unwrap()),
                    (_, _) => match (is_sender, is_receiver) {
                        (true, true) => TransactionType::SentAndReceived(receiver),
                        (true, false) => TransactionType::Sent(receiver),
                        (false, true) => TransactionType::Received(sender),
                        (false, false) => {
                            return Err(DBError::new(
                                "invalid state when converting TransactionDB to Transaction either sender or reciever must be group id",
                            ));
                        }
                    },
                }
            },
            money: money.into(),
            description,
            timestamp,
            is_undone_signal: RwSignal::new(is_undone), // might fail on server
        })
    }
}
