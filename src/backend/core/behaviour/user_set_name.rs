#![cfg(feature = "ssr")]

use sqlx::Executor;

use crate::backend::{
    core::User,
    database::{DatabaseResponse, DatabaseType, UserDB},
};
impl User {
    pub async fn set_name<T>(&mut self, conn: &mut T, new_value: String) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        UserDB::set_name(&mut *conn, self.id, new_value.clone()).await?;

        self.nickname = new_value;

        Ok(())
    }
}
