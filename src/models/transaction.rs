use chrono::{DateTime, Utc};
use leptos::prelude::RwSignal;

use crate::model::{GroupId, Money};
#[cfg(feature = "ssr")]
use crate::{
    backend::database::TransactionDB,
    model::{Page, PageRequestParams, UserId},
};

use super::DatabaseId;

#[cfg(feature = "ssr")]
use {
    crate::backend::database::DBError,
    crate::backend::database::{DatabaseResponse, DatabaseType},
    crate::backend::database::{DBGROUP_AUFLADUNG_ID, DBGROUP_SNACKBAR_ID},
    itertools::Itertools,
    sqlx::query,
    sqlx::Executor,
};

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

use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
impl From<Transaction> for TransactionDB {
    fn from(value: Transaction) -> Self {
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
        use crate::backend::database::DBGROUP_AUFLADUNG_ID;

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
                use crate::backend::database::DBGROUP_SNACKBAR_ID;
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

#[cfg(feature = "ssr")]
impl TransactionDB {
    pub async fn get_user_transactions<T>(
        conn: &mut T,
        user_id: UserId,
        page_request_params: PageRequestParams,
    ) -> DatabaseResponse<Page<Self>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let result = sqlx::query_as::<_, Self>(
            "
            select Transactions.* from Transactions
            join Users on Users.id=?
            join UserGroupMap as UGM on UGM.uid = Users.id
            where Transactions.receiver = UGM.gid or Transactions.sender = UGM.gid
            order by timestamp desc
            limit ?
            offset ?
        ",
        )
        .bind(user_id.0)
        .bind(page_request_params.limit as i64)
        .bind(page_request_params.offset as i64);

        let result = result
            .fetch_all(&mut *conn)
            .await
            .map_err(Into::<DBError>::into)?;

        let count = sqlx::query_as::<_, (u64,)>(
            r#"
                select count(*) from Transactions
            join Users on Users.id=?
            join UserGroupMap as UGM on UGM.uid = Users.id
            where Transactions.receiver = UGM.gid or Transactions.sender = UGM.gid
            "#,
        )
        .bind(user_id.0)
        .fetch_one(&mut *conn)
        .await
        .map_err(Into::<DBError>::into)?;

        Ok(Page::new(page_request_params, count.0 as usize, result))
    }

    pub async fn set_money<T>(conn: &mut T, id: DatabaseId, new_value: i64) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        _ = query!(
            "
                update Transactions
                set money = ?
                where id = ?
            ",
            new_value,
            id
        )
        .execute(&mut *conn)
        .await
        .map_err(DBError::new)?;

        Ok(())
    }

    pub async fn set_undone<T>(conn: &mut T, id: i64, new_value: bool) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        _ = query!(
            "
                update Transactions
                set is_undone = ?
                where id = ?
            ",
            new_value,
            id
        )
        .execute(&mut *conn)
        .await
        .map_err(DBError::new)?;

        Ok(())
    }
}

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
