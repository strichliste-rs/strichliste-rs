#![cfg(feature = "ssr")]

use crate::model::DatabaseId;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct UserPreferencesDB {
    pub user_id: DatabaseId,
    pub alternative_coloring: bool,
}
