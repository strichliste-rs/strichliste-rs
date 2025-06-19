use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::backend::db::DatabaseId;

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
pub struct ArticleCostMapDB {
    article_id: DatabaseId,
    cost: i64,
    effective_since: DateTime<Utc>,
}
