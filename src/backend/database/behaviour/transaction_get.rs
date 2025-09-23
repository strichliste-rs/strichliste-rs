#![cfg(feature = "ssr")]

use sqlx::{query_as, Executor};

use crate::{backend::database::{
    DBError,  DatabaseResponse, DatabaseType, TransactionDB,
}, model::DatabaseId};

use chrono::{DateTime, Utc};

impl TransactionDB {
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
}
