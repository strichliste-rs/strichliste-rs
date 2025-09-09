#[cfg(feature = "ssr")]
use {
    crate::models::DatabaseId,
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
};

#[allow(unused)] //clippy cannot find its use in the db schema + sql query
#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
pub struct ArticleCostMapDB {
    article_id: DatabaseId,
    cost: i64,
    effective_since: DateTime<Utc>,
}
