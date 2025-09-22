#![cfg(feature = "ssr")]

use sqlx::{query, Executor};

use crate::{
    backend::database::{DBError, DatabaseType, UserDB},
    model::UserId,
};
impl UserDB {
    pub async fn set_name<T>(conn: &mut T, id: UserId, new_value: String) -> Result<(), DBError>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query!(
            "
                update Users
                set nickname = ?
                where id = ?
            ",
            new_value,
            id.0
        )
        .execute(&mut *conn)
        .await
        .map_err(From::from)
        .map(|_| ())
    }
}
