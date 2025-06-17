use std::str::FromStr;

use chrono::{DateTime, Local, Utc};
use leptos::prelude::RwSignal;

use super::Money;

#[cfg(feature = "ssr")]
use {
    crate::backend::db::{DBError, DB},
    sqlx::query,
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

#[cfg_attr(feature = "ssr", derive(sqlx::Type, sqlx::FromRow))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TransactionDB {
    pub id: Option<i64>,
    pub user_id: i64,
    pub is_undone: bool,
    pub t_type: TransactionTypeDB,
    pub t_type_data: Option<i64>,
    pub money: i64,
    pub description: Option<String>,
    pub timestamp: DateTime<Utc>,
}

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
    pub async fn add_to_db(&mut self, db: &DB) -> Result<(), DBError> {
        let mut conn = db.get_conn().await?;

        let result = query!(
            "
                insert into Transactions
                    (user_id, t_type, is_undone, t_type_data, money, description, timestamp)
                values
                    (?, ?, ?, ?, ?, ?, ?)
                returning id
            ",
            self.user_id,
            self.t_type,
            self.is_undone,
            self.t_type_data,
            self.money,
            self.description,
            self.timestamp,
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| DBError::new(e.to_string()))?;

        self.id = Some(result.id);

        Ok(())
    }

    pub async fn get_user_transactions(
        db: &DB,
        user_id: i64,
        limit: i64,
    ) -> Result<Vec<TransactionDB>, DBError> {
        let mut conn = db.get_conn().await?;

        let result = sqlx::query_as::<_, TransactionDB>(
            "
                select *
                from Transactions
                where user_id = ?
                order by timestamp desc
                limit ?
            ",
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| DBError::new(e.to_string()))?;

        Ok(result)
    }

    pub async fn get_by_id(db: &DB, id: i64) -> Result<Option<TransactionDB>, DBError> {
        let mut conn = db.get_conn().await?;

        let result = sqlx::query_as::<_, TransactionDB>(
            "
                select *
                from Transactions
                where id = ?
            ",
        )
        .bind(id)
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| DBError::new(e.to_string()))?;

        Ok(result)
    }

    /// Will update every field in the db except 'id' and 'user_id'
    pub async fn update(&self, db: &DB) -> Result<(), DBError> {
        let mut conn = db.get_conn().await?;
        let id = self.id.unwrap();
        _ = query!(
            "
                update Transactions
                set is_undone = ?, t_type = ?, t_type_data = ?, money = ?, description = ?, timestamp = ?
                where id = ?
            ",
            self.is_undone,
            self.t_type,
            self.t_type_data,
            self.money,
            self.description,
            self.timestamp,
            id
        ).execute(&mut *conn).await.map_err(|e| DBError::new(e.to_string()))?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub id: Option<i64>,
    pub user_id: i64,
    pub is_undone: bool,
    pub t_type: TransactionType,
    pub money: Money,
    pub description: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub is_undone_signal: RwSignal<bool>,
}
