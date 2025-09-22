#[cfg(feature = "ssr")]
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use leptos::prelude::RwSignal;

use crate::model::{GroupId, Money};
#[cfg(feature = "ssr")]
use crate::{
    backend::{
        core::{Group, Settings, User},
        database::TransactionDB,
    },
    model::{Page, PageRequestParams, UserId},
    routes::user::CreateTransactionError,
};

use super::DatabaseId;

#[cfg(feature = "ssr")]
use {
    crate::backend::database::DBError,
    crate::backend::database::{DatabaseResponse, DatabaseType},
    crate::backend::database::{DBGROUP_AUFLADUNG_ID, DBGROUP_SNACKBAR_ID},
    itertools::Itertools,
    sqlx::query,
    sqlx::{query_as, Executor},
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
struct TransactionDelta {
    amount_pre: i64,
    delta: i64,
}

#[cfg(feature = "ssr")]
impl TransactionDelta {
    fn post_amount(&self) -> i64 {
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
    pub async fn create<T>(
        conn: &mut T,
        sender: GroupId,
        receiver: GroupId,
        t_type_data: Option<i64>,
        description: Option<String>,
        money: i64,
    ) -> DatabaseResponse<DatabaseId>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let now = Utc::now();
        query!(
            "
                insert into Transactions
                    (receiver, sender, is_undone, t_type_data, money, description, timestamp)
                values
                    (?, ?, ?, ?, ?, ?, ?)
                returning id
            ",
            receiver.0,
            sender.0,
            false,
            t_type_data,
            money,
            description,
            now
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(From::from)
        .map(|elem| elem.id)
    }

    pub async fn get<T>(conn: &mut T, id: DatabaseId) -> DatabaseResponse<Option<TransactionDB>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let result = query_as!(
            TransactionDB,
            r#"
                select
                    id as "id: i64",
                    sender as "sender: i64",
                    receiver as "receiver: i64",
                    is_undone,
                    t_type_data,
                    money as "money: u64",
                    description,
                    timestamp as "timestamp: DateTime<Utc>"
                from Transactions
                where id = ?
            "#,
            id
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(DBError::new)?;

        let result = match result {
            None => return Ok(None),
            Some(value) => value,
        };

        Ok(Some(result))
    }

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

#[cfg(feature = "ssr")]
impl Transaction {
    async fn get_transaction_delta<T>(
        conn: &mut T,
        sender_group: &Group,
        receiver_group: &Group,
        transaction_db: &TransactionDB,
    ) -> Result<HashMap<User, TransactionDelta>, DBError>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        use tracing::error;

        let mut senders = Vec::<User>::new();
        let mut receivers = Vec::<User>::new();

        for sender in sender_group.members.iter() {
            let user_send = match User::get(&mut *conn, UserId(sender.id)).await? {
                Some(val) => val,
                None => {
                    error!("Failed to find a user that should exist! Id: {}", sender.id);
                    return Err(DBError::new("Failed to find user"));
                }
            };

            senders.push(user_send);
        }

        for receiver in receiver_group.members.iter() {
            let user_recv = match User::get(&mut *conn, UserId(receiver.id)).await? {
                Some(val) => val,
                None => {
                    error!(
                        "Failed to find a user that should exist! Id: {}",
                        receiver.id
                    );
                    return Err(DBError::new("Failed to find user"));
                }
            };

            receivers.push(user_recv);
        }

        let mut delta_map = HashMap::new();

        let mut full_cost = transaction_db.money;
        let cost_share = transaction_db.money / sender_group.members.len() as u64;

        for user in senders.iter().chain(receivers.iter()) {
            _ = delta_map.insert(
                user.clone(),
                TransactionDelta {
                    amount_pre: user.money.value,
                    delta: 0,
                },
            );
        }

        for sender in senders.iter() {
            let user = match delta_map.get_mut(sender) {
                Some(user) => user,
                None => {
                    error!("Failed to find user in HashMap where it should exist!");
                    return Err(DBError::new("Failed to find user"));
                }
            };

            user.delta -= cost_share as i64;
            full_cost -= cost_share;
        }

        while full_cost > 0 {
            for sender in senders.iter_mut() {
                let user = match delta_map.get_mut(sender) {
                    Some(user) => user,
                    None => {
                        error!("Failed to find user in HashMap where it should exist!");
                        return Err(DBError::new("Failed to find user"));
                    }
                };
                user.delta -= 1;
                if full_cost == 0 {
                    break;
                }

                full_cost -= 1;
            }
        }

        let cost_share = transaction_db.money / receiver_group.members.len() as u64;

        for receiver in receivers.iter_mut() {
            let user = match delta_map.get_mut(receiver) {
                Some(user) => user,
                None => {
                    error!("Failed to find user in HashMap where it should exist!");
                    return Err(DBError::new("Failed to find user"));
                }
            };
            user.delta += cost_share as i64;
            full_cost += cost_share;
        }

        while full_cost < transaction_db.money {
            for receiver in receivers.iter_mut() {
                let user = match delta_map.get_mut(receiver) {
                    Some(user) => user,
                    None => {
                        error!("Failed to find user in HashMap where it should exist!");
                        return Err(DBError::new("Failed to find user"));
                    }
                };
                user.delta += 1;
                if full_cost == transaction_db.money {
                    break;
                }

                full_cost += 1;
            }
        }

        Ok(delta_map)
    }

    pub async fn create<T>(
        conn: &mut T,
        sender: GroupId,
        receiver: GroupId,
        t_type: TransactionType,
        description: Option<String>,
        money: Money,
        settings: &Settings,
    ) -> Result<DatabaseId, CreateTransactionError>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        type Error = CreateTransactionError;

        let t_type_data = match t_type {
            TransactionType::Bought(id) => Some(id),
            TransactionType::Received(id) => Some(id.0),

            _ => None,
        };

        let t_id = TransactionDB::create(
            &mut *conn,
            sender,
            receiver,
            t_type_data,
            description,
            money.value,
        )
        .await?;

        let transaction_db = match TransactionDB::get(&mut *conn, t_id).await? {
            Some(val) => val,
            None => return Err(Error::new("Failed to find newly created transaction")),
        };

        let (sender_group, receiver_group) = (
            Group::get(&mut *conn, GroupId(transaction_db.sender)).await?,
            Group::get(&mut *conn, GroupId(transaction_db.receiver)).await?,
        );

        let deltas = Transaction::get_transaction_delta(
            &mut *conn,
            &sender_group,
            &receiver_group,
            &transaction_db,
        )
        .await?;

        let mut users_too_low = Vec::<String>::new();
        let mut users_too_high = Vec::<String>::new();

        for (key, value) in deltas.iter() {
            use crate::backend::database::{DBUSER_AUFLADUNG_ID, DBUSER_SNACKBAR_ID};

            if key.id.0 == DBUSER_AUFLADUNG_ID.0 || key.id.0 == DBUSER_SNACKBAR_ID.0 {
                // don't do tracking on the system users
                continue;
            }

            if value.post_amount() > settings.accounts.upper_limit {
                if value.delta < 0 {
                    // allow users to loose money
                    continue;
                }

                users_too_high.push(key.nickname.clone());
            } else if value.post_amount() < settings.accounts.lower_limit {
                if value.delta > 0 {
                    // allow users to get money
                    continue;
                }

                users_too_low.push(key.nickname.clone());
            }
        }

        if !users_too_low.is_empty() {
            return Err(CreateTransactionError::TooLittleMoneyError(users_too_low));
        }

        if !users_too_high.is_empty() {
            return Err(CreateTransactionError::TooMuchMoneyError(users_too_high));
        }

        for (mut key, value) in deltas.into_iter() {
            key.add_money(&mut *conn, Money { value: value.delta })
                .await?;
        }

        Ok(transaction_db.id)
    }
}
