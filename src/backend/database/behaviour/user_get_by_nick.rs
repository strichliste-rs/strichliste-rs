#![cfg(feature = "ssr")]

use sqlx::{query_as, Executor};

use crate::backend::database::{DatabaseResponse, DatabaseType, UserDB};
impl UserDB {
    pub async fn get_by_nick<T>(conn: &mut T, nick: &String) -> DatabaseResponse<Option<Self>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query_as!(
            UserDB,
            "
                select *
                from Users
                where nickname = ?
            ",
            nick
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(From::from)
    }
}
