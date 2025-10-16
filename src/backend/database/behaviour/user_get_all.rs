#![cfg(feature = "ssr")]

use sqlx::{query_as, Executor};

use crate::backend::database::{DatabaseResponse, DatabaseType, UserDB};
use chrono::{DateTime, Utc};

impl UserDB {
    pub async fn get_all<T>(conn: &mut T) -> DatabaseResponse<Vec<Self>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query_as!(
            UserDB,
            r#"
                select
                    id,
                    nickname,
                    money,
                    is_system_user,
                    created_at as "created_at: DateTime<Utc>",
                    disabled
                from
                    Users
                where
                    is_system_user = false and
                    disabled = false
            "#,
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(From::from)
    }
}
