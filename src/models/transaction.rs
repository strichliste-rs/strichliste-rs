use std::str::FromStr;

use chrono::{DateTime, Local, Utc};
use leptos::prelude::RwSignal;

use super::{Article, Money};

#[cfg(feature = "ssr")]
use {
    super::ArticleDB,
    crate::backend::db::{DBError, DB},
    crate::backend::db::{DatabaseId, DatabaseResponse, DatabaseType},
    sqlx::query,
    sqlx::{query_as, Executor},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TransactionType {
    DEPOSIT,
    WITHDRAW,
    BOUGTH(i64),
    RECEIVED(i64),
    SENT(i64),
}

#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TransactionTypeDB {
    DEPOSIT,
    WITHDRAW,
    BOUGTH,
    RECEIVED,
    SENT,
}

impl From<&TransactionType> for TransactionTypeDB {
    fn from(value: &TransactionType) -> Self {
        match value {
            TransactionType::DEPOSIT => Self::DEPOSIT,
            TransactionType::WITHDRAW => Self::WITHDRAW,
            TransactionType::BOUGTH(_) => Self::BOUGTH,
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
            TransactionType::BOUGTH(_) => Self::BOUGTH,
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
            TransactionTypeDB::BOUGTH => Self::BOUGTH(value.1.unwrap()),
            TransactionTypeDB::RECEIVED => Self::RECEIVED(value.1.unwrap()),
            TransactionTypeDB::SENT => Self::SENT(value.1.unwrap()),
        }
    }
}

impl From<(TransactionTypeDB, Option<i64>)> for TransactionType {
    fn from(value: (TransactionTypeDB, Option<i64>)) -> Self {
        match value.0 {
            TransactionTypeDB::DEPOSIT => Self::DEPOSIT,
            TransactionTypeDB::WITHDRAW => Self::WITHDRAW,
            TransactionTypeDB::BOUGTH => Self::BOUGTH(value.1.unwrap()),
            TransactionTypeDB::RECEIVED => Self::RECEIVED(value.1.unwrap()),
            TransactionTypeDB::SENT => Self::SENT(value.1.unwrap()),
        }
    }
}

use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
pub struct TransactionDB {
    pub id: i64,
    pub user_id: i64,
    pub is_undone: bool,
    pub t_type: TransactionTypeDB,
    pub t_type_data: Option<i64>,
    pub money: i64,
    pub description: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[cfg(feature = "ssr")]
impl From<&Transaction> for TransactionDB {
    fn from(value: &Transaction) -> Self {
        let Transaction {
            id,
            user_id,
            is_undone,
            t_type,
            money,
            description,
            timestamp,
            is_undone_signal: _,
        } = value;

        TransactionDB {
            id: *id,
            user_id: *user_id,
            is_undone: *is_undone,
            t_type_data: match value.t_type {
                TransactionType::SENT(var)
                | TransactionType::BOUGTH(var)
                | TransactionType::RECEIVED(var) => Some(var),
                _ => None,
            },
            t_type: t_type.into(),
            money: (*money).value,
            description: description.clone(),
            timestamp: *timestamp,
        }
    }
}

#[cfg(feature = "ssr")]
impl From<Transaction> for TransactionDB {
    fn from(value: Transaction) -> Self {
        let Transaction {
            id,
            user_id,
            is_undone,
            t_type,
            money,
            description,
            timestamp,
            is_undone_signal,
        } = value;

        Self {
            id,
            user_id,
            is_undone,
            t_type_data: match t_type {
                TransactionType::SENT(var)
                | TransactionType::BOUGTH(var)
                | TransactionType::RECEIVED(var) => Some(var),
                _ => None,
            },
            t_type: t_type.into(),
            money: money.value,
            description,
            timestamp,
        }
    }
}

#[cfg(feature = "ssr")]
impl Into<Transaction> for TransactionDB {
    fn into(self) -> Transaction {
        Transaction {
            id: self.id,
            user_id: self.user_id,
            is_undone: self.is_undone,
            t_type: (self.t_type, self.t_type_data).into(),
            money: self.money.into(),
            description: self.description,
            timestamp: self.timestamp,
            is_undone_signal: RwSignal::new(self.is_undone), // might fail on server
        }
    }
}

#[cfg(feature = "ssr")]
impl TransactionDB {
    pub async fn create<T>(
        conn: &mut T,
        user_id: DatabaseId,
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
                    (user_id, t_type, is_undone, t_type_data, money, description, timestamp)
                values
                    (?, ?, ?, ?, ?, ?, ?)
                returning id
            ",
            user_id,
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
                    user_id as "user_id: i64",
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
        user_id: DatabaseId,
        limit: i64,
    ) -> DatabaseResponse<Vec<Self>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let result = query_as!(
            Self,
            r#"
            select
                id as "id: i64",
                user_id as "user_id: i64",
                is_undone,
                t_type as "t_type: TransactionTypeDB",
                t_type_data,
                money,
                description,
                timestamp as "timestamp: DateTime<Utc>"
            from Transactions
            where user_id = ?
            order by timestamp desc
            limit ?
        "#,
            user_id,
            limit
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
    pub id: i64,
    pub user_id: i64,
    pub is_undone: bool,
    pub t_type: TransactionType,
    pub money: Money,
    pub description: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub is_undone_signal: RwSignal<bool>,
}

#[cfg(feature = "ssr")]
impl Transaction {
    pub async fn get<T>(conn: &mut T, id: DatabaseId) -> DatabaseResponse<Option<Self>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let transaction_db = TransactionDB::get(conn, id).await?;

        let transaction_db = match transaction_db {
            Some(value) => value,
            None => return Ok(None),
        };

        let transaction: Transaction = transaction_db.into();
        Ok(Some(transaction))
    }

    pub async fn get_user_transactions(
        db: &DB,
        user_id: DatabaseId,
        limit: i64,
    ) -> DatabaseResponse<Vec<Self>> {
        let mut conn = db.get_conn().await?;

        let mut transactions = TransactionDB::get_user_transactions(&mut *conn, user_id, limit)
            .await?
            .into_iter()
            .map(|elem| elem.into())
            .collect::<Vec<Transaction>>();

        for transaction in transactions.iter_mut() {
            match transaction.t_type {
                TransactionType::BOUGTH(article_id) => {
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

                    transaction.money = (-price).into();
                    transaction.description = Some(article.name);
                }

                _ => {}
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
        user_id: DatabaseId,
        t_type: TransactionType,
        description: Option<String>,
        money: Money,
    ) -> DatabaseResponse<Self>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let t_type_data = match t_type {
            TransactionType::BOUGTH(id)
            | TransactionType::RECEIVED(id)
            | TransactionType::SENT(id) => Some(id),

            _ => None,
        };

        let t_id = TransactionDB::create(
            &mut *conn,
            user_id,
            t_type.into(),
            t_type_data,
            description,
            money.value,
        )
        .await?;

        Ok(Transaction::get(&mut *conn, t_id)
            .await?
            .expect("Newly created transaction should be present"))
    }
}
