use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use {
    crate::backend::db::DatabaseId,
    crate::backend::db::{DBError, DB},
    sqlx::{query, query_as},
};

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
pub struct ArticleCostMapDB {
    article_id: DatabaseId,
    cost: i64,
    effective_since: DateTime<Utc>,
}
