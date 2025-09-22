#![cfg(feature = "ssr")]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
pub struct TransactionDB {
    pub id: i64,
    pub sender: i64,
    pub receiver: i64,
    pub is_undone: bool,
    pub t_type_data: Option<i64>,
    pub money: u64,
    pub description: Option<String>,
    pub timestamp: DateTime<Utc>,
}
