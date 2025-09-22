#![cfg(feature = "ssr")]

use sqlx::Executor;

use crate::backend::{
    core::User,
    database::{DatabaseResponse, DatabaseType, UserDB},
};

impl User {
    pub async fn set_money<T>(&mut self, conn: &mut T, new_value: i64) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        UserDB::set_money(&mut *conn, self.id, new_value).await
    }
}
