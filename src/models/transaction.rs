use std::collections::HashMap;

use chrono::{DateTime, Utc};
use leptos::prelude::RwSignal;

use super::{DatabaseId, GroupId, Money, UserId};

#[cfg(feature = "ssr")]
use {
    super::ArticleDB,
    crate::backend::db::{DBError, DB},
    crate::backend::db::{DatabaseResponse, DatabaseType},
    crate::models::{Group, GroupDB},
    sqlx::query,
    sqlx::{query_as, Executor},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TransactionType {
    DEPOSIT,
    WITHDRAW,
    BOUGHT(i64),
    RECEIVED(GroupId),
    SENT(GroupId),
}

#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TransactionTypeDB {
    DEPOSIT,
    WITHDRAW,
    BOUGHT,
    RECEIVED,
    SENT,
}

impl From<&TransactionType> for TransactionTypeDB {
    fn from(value: &TransactionType) -> Self {
        match value {
            TransactionType::DEPOSIT => Self::DEPOSIT,
            TransactionType::WITHDRAW => Self::WITHDRAW,
            TransactionType::BOUGHT(_) => Self::BOUGHT,
            TransactionType::RECEIVED(_) => Self::RECEIVED,
            TransactionType::SENT(_) => Self::SENT,
        }
    }
}

impl From<TransactionType> for TransactionTypeDB {
    fn from(value: TransactionType) -> Self {
        match value {
            TransactionType::DEPOSIT => Self::DEPOSIT,
            TransactionType::WITHDRAW => Self::WITHDRAW,
            TransactionType::BOUGHT(_) => Self::BOUGHT,
            TransactionType::RECEIVED(_) => Self::RECEIVED,
            TransactionType::SENT(_) => Self::SENT,
        }
    }
}

impl From<(&TransactionTypeDB, Option<i64>)> for TransactionType {
    fn from(value: (&TransactionTypeDB, Option<i64>)) -> Self {
        match value.0 {
            TransactionTypeDB::DEPOSIT => Self::DEPOSIT,
            TransactionTypeDB::WITHDRAW => Self::WITHDRAW,
            TransactionTypeDB::BOUGHT => Self::BOUGHT(value.1.unwrap()),
            TransactionTypeDB::RECEIVED => Self::RECEIVED(value.1.unwrap().into()),
            TransactionTypeDB::SENT => Self::SENT(value.1.unwrap().into()),
        }
    }
}

impl From<(TransactionTypeDB, Option<i64>)> for TransactionType {
    fn from(value: (TransactionTypeDB, Option<i64>)) -> Self {
        match value.0 {
            TransactionTypeDB::DEPOSIT => Self::DEPOSIT,
            TransactionTypeDB::WITHDRAW => Self::WITHDRAW,
            TransactionTypeDB::BOUGHT => Self::BOUGHT(value.1.unwrap()),
            TransactionTypeDB::RECEIVED => Self::RECEIVED(value.1.unwrap().into()),
            TransactionTypeDB::SENT => Self::SENT(value.1.unwrap().into()),
        }
    }
}

use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
pub struct TransactionDB {
    pub id: i64,
    pub sender: i64,
    pub receiver: i64,
    pub is_undone: bool,
    pub t_type: TransactionTypeDB,
    pub t_type_data: Option<i64>,
    pub money: i64,
    pub description: Option<String>,
    pub timestamp: DateTime<Utc>,
}

// #[cfg(feature = "ssr")]
// impl From<&Transaction> for TransactionDB {
//     fn from(value: &Transaction) -> Self {
//         let Transaction {
//             id,
//             user_id,
//             is_undone,
//             t_type,
//             money,
//             description,
//             timestamp,
//             is_undone_signal: _,
//         } = value;

//         TransactionDB {
//             id: *id,
//             user_id: *user_id,
//             is_undone: *is_undone,
//             t_type_data: match value.t_type {
//                 TransactionType::SENT(var)
//                 | TransactionType::BOUGHT(var)
//                 | TransactionType::RECEIVED(var) => Some(var),
//                 _ => None,
//             },
//             t_type: t_type.into(),
//             money: (*money).value,
//             description: description.clone(),
//             timestamp: *timestamp,
//         }
//     }
// }

// #[cfg(feature = "ssr")]
// impl From<Transaction> for TransactionDB {
//     fn from(value: Transaction) -> Self {
//         let Transaction {
//             id,
//             user_id,
//             is_undone,
//             t_type,
//             money,
//             description,
//             timestamp,
//             is_undone_signal: _,
//         } = value;

//         Self {
//             id,
//             user_id,
//             is_undone,
//             t_type_data: match t_type {
//                 TransactionType::SENT(var)
//                 | TransactionType::BOUGHT(var)
//                 | TransactionType::RECEIVED(var) => Some(var),
//                 _ => None,
//             },
//             t_type: t_type.into(),
//             money: money.value,
//             description,
//             timestamp,
//         }
//     }
// }

#[cfg(feature = "ssr")]
/// Use the GroupId (self.1) if the user is the only person relevant in the
/// transaction
impl Into<Transaction> for (TransactionDB, GroupId) {
    fn into(self: (TransactionDB, GroupId)) -> Transaction {
        let TransactionDB {
            id,
            sender,
            receiver,
            is_undone,
            t_type,
            t_type_data,
            money,
            description,
            timestamp,
        } = self.0;

        Transaction {
            id,
            group_id: match t_type {
                TransactionTypeDB::WITHDRAW => self.1,
                TransactionTypeDB::DEPOSIT => self.1,
                TransactionTypeDB::BOUGHT => self.1,
                TransactionTypeDB::RECEIVED => sender.into(),
                TransactionTypeDB::SENT => receiver.into(),
            },
            is_undone,
            t_type: (t_type, t_type_data).into(),
            money: money.into(),
            description,
            timestamp,
            is_undone_signal: RwSignal::new(is_undone), // might fail on server
        }
    }
}

#[cfg(feature = "ssr")]
impl TransactionDB {
    pub async fn create<T>(
        conn: &mut T,
        sender: GroupId,
        receiver: GroupId,
        t_type: TransactionTypeDB,
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
                    (receiver, sender, t_type, is_undone, t_type_data, money, description, timestamp)
                values
                    (?, ?, ?, ?, ?, ?, ?, ?)
                returning id
            ",
            receiver.0,
            sender.0,
            t_type,
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
                    t_type as "t_type: TransactionTypeDB",
                    t_type_data,
                    money,
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
        limit: i64,
        offset: i64,
    ) -> DatabaseResponse<Vec<Self>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let groups = GroupDB::get_groups(&mut *conn, user_id).await?;

        let string = groups
            .into_iter()
            .map(|group| format!("{}", group.id))
            .collect::<Vec<String>>()
            .join(", ");

        let result = query_as!(
            Self,
            r#"
            select
                id as "id: i64",
                sender as "sender: i64",
                receiver as "receiver: i64",
                is_undone,
                t_type as "t_type: TransactionTypeDB",
                t_type_data,
                money,
                description,
                timestamp as "timestamp: DateTime<Utc>"
            from Transactions
            where sender in (?) or receiver in (?)
            order by timestamp desc
            limit ?
            offset ?
        "#,
            string,
            string,
            limit,
            offset,
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(Into::<DBError>::into)?;

        Ok(result)
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
        // .map_err(From::<DBError>::from)?;
        .map_err(DBError::new)?;

        Ok(())
    }

    async fn set_undone<T>(conn: &mut T, id: i64, new_value: bool) -> DatabaseResponse<()>
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
    pub async fn get<T>(
        conn: &mut T,
        id: DatabaseId,
        user_id: UserId,
    ) -> DatabaseResponse<Option<Self>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let transaction_db = TransactionDB::get(conn, id).await?;

        let transaction_db = match transaction_db {
            Some(value) => value,
            None => return Ok(None),
        };

        let user_group_single = GroupDB::get_single_group(&mut *conn, user_id).await?;

        let transaction: Transaction = (transaction_db, user_group_single.into()).into();
        Ok(Some(transaction))
    }

    pub async fn get_user_transactions(
        db: &DB,
        user_id: DatabaseId,
        limit: i64,
        offset: i64,
    ) -> DatabaseResponse<Vec<Self>> {
        let mut conn = db.get_conn().await?;

        let user_id: UserId = user_id.into();

        let user_group_id = Group::get_user_group(&mut *conn, user_id).await?;

        let mut transactions =
            TransactionDB::get_user_transactions(&mut *conn, user_id, limit, offset)
                .await?
                .into_iter()
                .map(|elem| (elem, user_group_id).into())
                .collect::<Vec<Transaction>>();
        let mut article_cache = HashMap::<i64, (i64, String)>::new();

        for transaction in transactions.iter_mut() {
            if let TransactionType::BOUGHT(article_id) = transaction.t_type {
                let (price, article_name) = match article_cache.get(&article_id) {
                    None => {
                        let article = match ArticleDB::get_single(&mut *conn, article_id).await? {
                            None => continue, // Article got nuked?,
                            Some(value) => value,
                        };

                        let price = ArticleDB::get_effective_cost(
                            &mut *conn,
                            article_id,
                            transaction.timestamp,
                        )
                        .await?;

                        let result = (price, article.name);

                        _ = article_cache.insert(article_id, result.clone());

                        result
                    }

                    Some(value) => value.clone(),
                };

                transaction.money = (-price).into();
                transaction.description = Some(article_name);
            }
        }

        Ok(transactions)
    }

    pub async fn set_money<T>(&mut self, conn: &mut T, new_value: i64) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        TransactionDB::set_money(&mut *conn, self.id, new_value).await
    }

    pub async fn set_undone<T>(&mut self, conn: &mut T, new_value: bool) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        TransactionDB::set_undone(&mut *conn, self.id, new_value).await
    }

    pub async fn create<T>(
        conn: &mut T,
        sender: GroupId,
        receiver: GroupId,
        t_type: TransactionType,
        description: Option<String>,
        money: Money,
    ) -> DatabaseResponse<DatabaseId>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let t_type_data = match t_type {
            TransactionType::BOUGHT(id) => Some(id),
            TransactionType::RECEIVED(id) | TransactionType::SENT(id) => Some(id.0),

            _ => None,
        };

        let t_id = TransactionDB::create(
            &mut *conn,
            sender,
            receiver,
            t_type.into(),
            t_type_data,
            description,
            money.value,
        )
        .await?;

        Ok(t_id)
    }
}
