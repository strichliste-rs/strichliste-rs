#![cfg(feature = "ssr")]

use sqlx::{query_as, Executor};

use crate::backend::database::{DatabaseResponse, DatabaseType, UserDB};
use chrono::{DateTime, Utc};

impl UserDB {
    pub async fn get<T>(conn: &mut T, id: i64) -> DatabaseResponse<Option<Self>>
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
                from Users
                where id = ?
            "#,
            id
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(From::from)
    }
}
