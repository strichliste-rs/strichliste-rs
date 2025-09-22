#![cfg(feature = "ssr")]

use sqlx::{query_as, Executor};

use crate::backend::database::{DatabaseResponse, DatabaseType, UserDB};

impl UserDB {
    pub async fn get_all<T>(conn: &mut T) -> DatabaseResponse<Vec<Self>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query_as!(
            UserDB,
            "
                select *
                from Users
                where is_system_user = false
            ",
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(From::from)
    }
}
