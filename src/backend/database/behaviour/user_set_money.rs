#![cfg(feature = "ssr")]

use sqlx::{query, Executor};

use crate::{
    backend::database::{DBError, DatabaseResponse, DatabaseType, UserDB},
    model::UserId,
};

impl UserDB {
    pub async fn set_money<T>(conn: &mut T, user_id: UserId, new_value: i64) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        _ = query!(
            "
                update Users
                set money = ?
                where id = ?
            ",
            new_value,
            user_id.0
        )
        .execute(&mut *conn)
        .await
        .map_err(DBError::new)?;

        Ok(())
    }
}
