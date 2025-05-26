use std::str::FromStr;

use chrono::{DateTime, Local, Utc};

#[cfg(feature = "ssr")]
use {
    crate::backend::db::{DBError, DB},
    sqlx::query,
};

#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TransactionType {
    DEPOSIT,
    WITHDRAW,
    BOUGTH,
    RECEIVED,
    SENT,

    UNKNOWN,
}

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub id: Option<i64>,
    pub user_id: i64,
    pub t_type: TransactionType,
    pub origin_user: Option<i64>,
    pub destination_user: Option<i64>,
    pub money: i64,
    pub description: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            id: None,
            user_id: 0,
            t_type: TransactionType::UNKNOWN,
            origin_user: None,
            destination_user: None,
            money: 0,
            description: None,
            timestamp: Local::now().to_utc(),
        }
    }
}

#[cfg(feature = "ssr")]
impl Transaction {
    pub async fn add_to_db(&mut self, db: &DB) -> Result<(), DBError> {
        let mut conn = db.get_conn().await?;

        let result = query!(
            "
                insert into Transactions
                    (user_id, t_type, origin_user, destination_user, money, description, timestamp)
                values
                    (?, ?, ?, ?, ?, ?, ?)
                returning id
            ",
            self.user_id,
            self.t_type,
            self.origin_user,
            self.destination_user,
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
    ) -> Result<Vec<Transaction>, DBError> {
        let mut conn = db.get_conn().await?;

        let result = sqlx::query_as::<_, Transaction>(
            "
                select *
                from Transactions
                where user_id = ?
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
}
